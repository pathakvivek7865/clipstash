use crate::data::AppDatabase;
use crate::service::action;
use crate::service::{self};
use crate::web::{ctx, form, renderer::Renderer, PageError};
use crate::{ServiceError, Shortcode};
use rocket::form::{Contextual, Form};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::content::RawHtml;
use rocket::response::{status, Redirect};
use rocket::{uri, State};

use super::hitcounter::HitCounter;
use super::PASSWORD_COOKIE;

#[rocket::get("/")]
fn home(renderer: &State<Renderer<'_>>) -> RawHtml<String> {
    let context = ctx::Home::default();

    RawHtml(renderer.render(context, &[]))
}

#[rocket::post("/", data = "<form>")]
pub async fn new_clip(
    form: Form<Contextual<'_, form::NewClip>>,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,
) -> Result<Redirect, (Status, RawHtml<String>)> {
    let form = form.into_inner();
    if let Some(value) = form.value {
        let req = service::ask::NewClip {
            content: value.content,
            title: value.title,
            expires: value.expires,
            password: value.password,
        };

        match action::new_clip(req, database.get_pool()).await {
            Ok(clip) => Ok(Redirect::to(uri!(get_clip(shortcode = clip.shortcode)))),
            Err(err) => {
                eprintln!("internal server error: {}", err);
                Err((
                    Status::InternalServerError,
                    RawHtml(renderer.render(
                        ctx::Home::default(),
                        &["A server error occured, please try again"],
                    )),
                ))
            }
        }
    } else {
        let errors = form
            .context
            .errors()
            .map(|err| {
                use rocket::form::error::ErrorKind;

                if let ErrorKind::Validation(msg) = &err.kind {
                    msg.as_ref()
                } else {
                    eprintln!("unhandled error: {}", err);
                    "An error occured, please try again"
                }
            })
            .collect::<Vec<_>>();

        Err((
            Status::BadRequest,
            RawHtml(renderer.render_with_data(
                ctx::Home::default(),
                ("clip", &form.context),
                &errors,
            )),
        ))
    }
}

#[rocket::get("/clip/<shortcode>")]
pub async fn get_clip(
    shortcode: Shortcode,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,
    hit_counter: &State<HitCounter>,
) -> Result<status::Custom<RawHtml<String>>, PageError> {
    fn render_with_status<T>(
        status: Status,
        context: T,
        renderer: &Renderer,
    ) -> Result<status::Custom<RawHtml<String>>, PageError>
    where
        T: ctx::PageContext + serde::Serialize + std::fmt::Debug,
    {
        Ok(status::Custom(
            status,
            RawHtml(renderer.render(context, &[])),
        ))
    }

    match action::get_clip(shortcode.clone().into(), database.get_pool()).await {
        Ok(clip) => {
            hit_counter.hit(shortcode.clone(), 1);
            let context = ctx::ViewClip::new(clip);
            render_with_status(Status::Ok, context, renderer)
        }
        Err(err) => match err {
            ServiceError::PermissionError(_) => {
                let context = ctx::PasswordRequired::new(shortcode);
                render_with_status(Status::Unauthorized, context, renderer)
            }
            ServiceError::NotFound => Err(PageError::NotFound("Clip not found".to_owned())),
            _ => Err(PageError::Internal("server eror".to_owned())),
        },
    }
}

#[rocket::post("/clip/<shortcode>", data = "<form>")]
pub async fn submit_clip_password(
    shortcode: Shortcode,
    cookies: &CookieJar<'_>,
    form: Form<Contextual<'_, form::GetPasswordProtectedClip>>,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,
    hit_counter: &State<HitCounter>,
) -> Result<RawHtml<String>, PageError> {
    if let Some(form) = &form.value {
        let req = service::ask::GetClip {
            shortcode: shortcode.clone(),
            password: form.password.clone(),
        };

        match action::get_clip(req, database.get_pool()).await {
            Ok(clip) => {
                hit_counter.hit(shortcode.clone(), 1);
                let context = ctx::ViewClip::new(clip);
                cookies.add(Cookie::new(
                    PASSWORD_COOKIE,
                    form.password.clone().into_inner().unwrap_or_default(),
                ));
                Ok(RawHtml(renderer.render(context, &[])))
            }
            Err(err) => match err {
                ServiceError::PermissionError(e) => {
                    let context = ctx::PasswordRequired::new(shortcode);
                    Ok(RawHtml(renderer.render(context, &[e.as_str()])))
                }
                ServiceError::NotFound => Err(PageError::NotFound("Clip not found".to_owned())),
                _ => Err(PageError::Internal("server error".to_owned())),
            },
        }
    } else {
        let context = ctx::PasswordRequired::new(shortcode);
        Ok(RawHtml(renderer.render(
            context,
            &["A password is required to view this page"],
        )))
    }
}

#[rocket::get("/clip/raw/<shortcode>")]
pub async fn get_raw_clip(
    cookies: &CookieJar<'_>,
    shortcode: Shortcode,
    database: &State<AppDatabase>,
    hit_counter: &State<HitCounter>,
) -> Result<status::Custom<String>, Status> {
    use crate::domain::clip::field::Password;

    let req = service::ask::GetClip {
        shortcode: shortcode.clone(),
        password: cookies
            .get(PASSWORD_COOKIE)
            .map(|cookie| cookie.value())
            .and_then(|raw_password| Password::new(raw_password.to_string()).ok())
            .unwrap_or_default(),
    };

    match action::get_clip(req, database.get_pool()).await {
        Ok(clip) => {
            hit_counter.hit(shortcode.clone(), 1);
            Ok(status::Custom(
                Status::Ok,
                serde_json::to_value(clip).unwrap_or_default().to_string(),
            ))
        }
        Err(err) => match err {
            ServiceError::PermissionError(msg) => Ok(status::Custom(Status::Unauthorized, msg)),
            ServiceError::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        },
    }
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home, get_clip, new_clip, submit_clip_password, get_raw_clip]
}

pub mod catcher {
    use rocket::Request;
    use rocket::{catch, catchers, Catcher};

    #[catch(default)]
    fn default(req: &Request) -> &'static str {
        eprintln!("General error: {:?}", req);
        "something went wrong..."
    }

    #[catch(500)]
    fn internal_err(req: &Request) -> &'static str {
        eprintln!("Inter error: {:?}", req);
        "Internal Server Error"
    }

    #[catch(404)]
    fn not_found() -> &'static str {
        "404"
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![not_found, default, internal_err]
    }
}
