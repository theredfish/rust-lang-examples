use actix_web::{web, App, HttpRequest, HttpServer, Responder, middleware::Logger};
use ldap3::{LdapConn};
use dotenv::dotenv;
use std::{ env, thread, time };
use env_logger;
use crossbeam_channel::{Sender, Receiver, unbounded};
use std::sync::Mutex;
use std::error::Error;

struct AppState {
  s: Sender<String>,
  r: Receiver<String>
}

impl AppState {
  pub fn new(s: Sender<String>, r: Receiver<String>) -> Self {
    AppState {s, r}
  }
}

// Before displaying the username, we check if the user
// is authenticated.
fn index(state: web::Data<Mutex<AppState>>, req: HttpRequest) -> impl Responder {
    let state_lock = state.lock().unwrap();
    state_lock.s.send("I would like to authenticate the user please!".to_string()).unwrap();

    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

fn ldap_auth() -> Result<(), Box<Error>> {
  let user = &env::var("LDAP_USER").expect("Cannot read the LDAP_USER variable in .env");
  let pwd = &env::var("LDAP_PASSWORD").expect("Cannot read the LDAP_PASSWORD variable in .env");
  let ldap_addr = &env::var("LDAP").expect("Cannot read the LDAP variable in .env");

  let ldap = LdapConn::new(ldap_addr)?;

  // Return a broken pipe error
  let simple_bind = match ldap.simple_bind(user, pwd) {
    Ok(simple_bind) => simple_bind,
    Err(e) =>  {
      eprintln!("Cannot create simple_bind : {}", e);
      return Err(Box::new(e));
    }
  };

  match simple_bind.success() {
    Ok(success) => success,
    Err(e) =>  {
      eprintln!("Not a successful simple_bind : {}", e);
      return Err(Box::new(e));
    }
  };

  // ldap.simple_bind(user, pwd)?.success()?;

  Ok(())
}

// Infinite loop listening on messages
fn check_auth(state: web::Data<Mutex<AppState>>) {
  thread::spawn(move || {
    loop {
        // let the time to the others resources to access to the lock... be fair!
        // We need to find a good shared resource strategy...
        thread::sleep(time::Duration::from_secs(3));
        let state_lock = state.lock().unwrap();

        if let Ok(message) = state_lock.r.try_recv() {
          println!("Message: {}", message);

          if ldap_auth().is_ok() {
            println!("Successful auth");
          } else {
            println!("Auth failed");
          }
        } else {
          println!("Waiting for a new message");
        }
      }
  });
}

fn main() {
  dotenv().ok();
  env_logger::init();

  // This is where we initialize our sender / receiver
  // inside an AppState
  let (s, r) = unbounded::<String>();
  let app_state = AppState::new(s,r);
  let state_mtx = web::Data::new(Mutex::new(app_state));

  // start the thread responsible of the authentication
  check_auth(state_mtx.clone());

  // start the http server
  HttpServer::new(move || {
      App::new()
        .register_data(state_mtx.clone())
        .wrap(Logger::default())
        .route("/", web::get().to(index))
  })
  .bind("127.0.0.1:8000")
  .expect("Can not bind to port 8000")
  .run()
  .unwrap();
}
