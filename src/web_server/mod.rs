use std::{
    fs,
    path::PathBuf,
    net::Ipv4Addr
};

use rocket::{
    routes,
    get,
    post,
    Rocket,
    Build,
    Config,
    response::content::RawHtml,
    http::{
        Cookie,
        CookieJar
    }
};



#[get("/<request_path..>?<token>")]
async fn index(request_path:PathBuf,token:Option<String>,cookies:&CookieJar<'_>)->Option<RawHtml<String>>{
    if let Some(token)=token{
        if token=="229gdss213sk3jci3cgd4d3fD"{
            cookies.add(Cookie::new("token","229gdss213sk3jci3cgd4d3fD"));
        }
        else{
            return None
        }
    }
    else{
        let Some(cookies)=cookies.get("token") else {return None};

        if cookies.value()!="229gdss213sk3jci3cgd4d3fD"{
            return None
        }
    }

    println!("RequestPath: {request_path:?}");

    let mut displayed_path=request_path.to_string_lossy().to_string();
    let mut path=PathBuf::from("resources");
    path.push(request_path);

    if !displayed_path.is_empty(){
        displayed_path.insert(0,'\\');
    }

    println!("DisplayedPath: {displayed_path:?}");
    println!("Path: {path:?}");

    if path.is_dir(){
        let Ok(directory_template)=fs::read_to_string("website/directory.html") else {return None};

        let mut files=String::new();

        let Ok(dir)=fs::read_dir(&path) else {return None};

        for entry in dir{
            let Ok(entry)=entry else {continue};

            let entry_name=entry.file_name();
            let name=entry_name.to_string_lossy();

            files+=&format!("<div><a href=\"{displayed_path}\\{}\">{}</a></div>",name,name)
        }

        Some(RawHtml(directory_template.replace("{{rust-input}}",&files)))
    }
    else{
        let Ok(file_template)=fs::read_to_string("website/file.html") else {return None};

        let Ok(text)=fs::read_to_string(path) else {return None};

        let mut formated_text=String::new();

        for line in text.lines(){
            formated_text+=&format!("<div>{line}</div>")
        }

        Some(RawHtml(file_template.replace("{{rust-input}}",&text)))
    }
}


#[post("/<request_path..>?<token>",data="<update>")]
async fn save(request_path:PathBuf,token:Option<String>,update:String,cookies:&CookieJar<'_>)->Option<&'static str>{
    println!("RequestPath: {request_path:?}");

    let mut path=PathBuf::from("resources");
    path.push(request_path);

    println!("UpdateData: {update:?}");

    if let Some(token)=&token{
        if token=="229gdss213sk3jci3cgd4d3fD"{
            cookies.add(Cookie::new("token","229gdss213sk3jci3cgd4d3fD"));
        }
        else{
            return None
        }
    }
    else{
        let Some(cookies)=cookies.get("token") else {return None};

        if cookies.value()!="229gdss213sk3jci3cgd4d3fD"{
            return None
        }
    }

    if path.is_file(){
        if fs::write(&path,update).is_err(){
            return None
        }
    }

    Some("OK")
}



pub struct WebServer{
    rocket:Rocket<Build>
}

impl WebServer{
    pub fn new()->WebServer{
        let config=Config{
            port:8080,
            address:Ipv4Addr::new(0,0,0,0).into(),
            ..Config::debug_default()
        };

        let rocket=rocket::custom(config).mount("/",routes![index]).mount("/",routes![save]);

        Self{
            rocket
        }
    }

    pub async fn run(self){
        self.rocket.launch().await.unwrap();
    }
}