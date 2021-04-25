pub mod libs;

#[macro_use] extern crate actix_web;
#[macro_use] extern crate serde_json;
use crate::libs::model::{
    link::{Link, LinkType},
    folder::Folder,
    group::Group,
    media::{Media, MediaType},
    page::Page,
    page_element::PageElement,
};
use anyhow::Result;
use libs::db;
use mongodb::bson::doc;
use actix_files as fs;
use actix_web::{App, HttpServer};

pub async fn _test_dummy_data_insertion() -> Result<()> {
    let connection = db::get_connection().await?;

    for db_name in connection.list_collection_names(None).await? {
        println!("{}", db_name);
    }

    let logo = "iVBORw0KGgoAAAANSUhEUgAAAE8AAABQCAYAAABYtCjIAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAu3SURBVHgB3V3rkeM4DuZs3f9zBs0M1hlYGbQzsC8C90UgZ+DJQH0ReDYCeSLwTATyReCeCL4TW6AFQnxJfs0eqlhtSSAJfgRBkALVX9QTCcCs/TNvk27Tn22a0W8tWM31R5tO7O+PNv00f798+fKhnkBf1AOJgfXapqUagjSVDJCHNv3VAnlQ/0/Ugla0qW7TGflkeI8T8jVtqtqk1Z3pbppHWvbWppXyaxgfhj/o94l+q1aD/uspT1FZL6rT3DmlEB3a9N6W9R/1dyDTyDZtI9pi7m/UjYjqW7SpJC0NaeNK/c6EbngeA8KXppHqzmSGa5vWASCrRwznUUS9/9UjbP0IwCJy7TAcAc1vo4WtIHMSSJIRfKaeTOiGdO2TTz2TWgHebq1tpMVrAl/TPU3XG3SmYXSnEIiNRwu1eiQhPExHTwTo7NDRDiUCypK9t/TU1WDC8ENne58DIDXu6BFgPrG8hsqo2D1rp3Z0vWF17eBq0IrlMxpbZNT5Ksq4P4DoNE4Cd8ypmBpWtWkvGry35bB7to49XVcenpru1XTNNfZNZZBoS3M3AAPAmYbPEvl8mgqrIeg0ydCZ5alsg+i65kDRPQd0lsdQls0l2Zq7A8iEtVRm5rPCfdo1AmtlgUenkZbmoi4LnhzGvNF7uj5LgDPl8wF4Oy+hLewdI4BjIBQcGAKr5uXQfS4419Id5YHg4b7bAu6svwoAVLdpGZBXjo5RHRADYiuE32Xw26HEgSno3h59T58JnNIDUEn85rmZMCq4mwQXMFh5TUCmWsrh4ZnBHfrX+YEY+nFRxxfMpWACnRkgGr1Tbe/zyeMFI4cMXO1uIDQPbufv2P1ZoDyugdNWInDtyGdPZ+ThQ+yF7vkcaUMlJro3AVkrCSKGJmFG/F8RmBxEu8+YMoF4hNERXuv9c+PLNaqEO9zusrYUIJZwJwJNncsnvmOgHO5XjrN/GHr0qwCfnABsbxm6eu1I5Ws1kjDUxBJ+l6mOlMF5s/xGm7HhFSeEXJOgDYaUrWHobNcuUpbdWd4TGEVGmTv0rkwjyrq0DR7zAdeW5g1fuH5Xo0YQA5PbynmE326cNoz/SACaobNiaeMB9kz3ikQdXItK9KPF2ERrk988eTcY4z8isG4UPAUiC3P0vbZKgMbtoBE0a6alRm+ErDXCboidBDZ0Dcq7FR0x8+QtGU+REsrSMcJXM74GAcc0kPcNrvty1WYphltNFfyzqJ1p+XDkbdCB8rm7VccE4UN2E+DxVR4EUQhRM/6s5V0uwdUQI8s8wFcJuZ2NDUzVvhyEBcBHDI279uThBtv8vcvWPIYTg29EcL+zgquRNTzDV3R87au4SKLb83IAt3TdgO3JBRrU4M77ZpkAvqPTptCbvo0nD2/zTD6s2EMdEMwUYHzAOSVb6Zae+3qseRRwrF4JYOHh2WFIZ7DtfzVsy9kLLqvsHBFIVtSwa18Pf011yL0IiWUWhpsRtoMrpO1lzW/yWbZKZLSVSFXXgj85+dyb4Nq3WjyzuygLdH6kpK2nvA3rjJm9ycFbeDLNPIVXlG8h88AdNkf1RELedlTDlKIicHwTx2CLTWrVa6ACuV7kIMpKtuy5VvHGGQ2920touMO3CdRvib/B88oFafeQMVkIYTh/7eFpQs8EH+/J0RsJ1Ek10u9RSkiN6Z/tc2Rl/HaiqeyNmm7sVSYxEOWQHfRkpIx1rBMSeQuW9zXBOw/Vg96OaZVX7/pSDlx7drXXzzriPEKQ0Zon8iblhusZzNj92Zi6GV7NH+11wZ79UFcQCVXQ5UGNo7F1/1ONo2/s90VTTUhum/6tMolCeE9tmhnwbrlU4v7RQY2jkxpHv9jvnN2Y7+z3tdv/B0Xg8YI+QtygjUUVJ94ROZp0srxtj35X0ymnrgP7rUNMZMdTw/i7ZebO7kuk0BqJzVG4s/CLyiBMeGNG+ewmZz3C2AddFsbTINNLUHDJtx3zDf2K4kzC7gKF1rYg9RsSIu93rXLQc9vObaSs+g9+I3CewVR0Up1dCZ2TuAmhc3+0eg4ZG8rbr1VgUmplXLd/3h3NUxHKHLb7qZqHfkkXWuEsceVrSyQiC4gHGcPWtHNuNM+ifVJxMj1zSPB8qOlktfoHupWDjXcxoBZUtvm7VtdTTM6Dct0aHxXKaCVc5/FVXUFw98hGuwPoo0RBWlag39Y6ki2a7GYw2SYH8qBfrSz+oTqN0/TMCPaXmk4/2W9zlmyU49va3H+RgDPKb8i4R3aC+jX1nJkA/aCm06UcC96t6MB+F22Knrwx9st3OocASvp9BIhZIZxUmjh416yk7Oj8xRfGhqoxpcDv2jQpo0x8Vv0LNYHAF+h5/DxGZbRfycppLmVAhGiNKETD/8Iny+6h39afZH/Q+5TJ2Rfu5sdePKsQeWEueC1WZ1/B2b2CfjVRBCowlAqEPPvKyKhbszp0Bv+a8b8GyvnsSMTfPVeDDocby5EET1Raep7X7HlM++w7hgYjhpIov0J6M7Sx9ahhO+TRBCvPKlLOjt/cJcAo0b0k0XRdsUq0h59rX8rhrC0IKoMwDPON1iH4ecygpnYt0QcSNRxAUQ4Pu3sNNdb34oM/r9jvjUqD4gjt4dMQke0BPh6YKOPsmkjZ3igIuEOZt90rs2j34KV3MKMn8wU89CFis4jg0fg2uO9FGvq9pmRDy+zGxIblsfF3ofesdah+uK8l7e6Md9cF7rzgDbcoowy9wDxmzQHSwy8FzDnwsiFAapYqdENrjF30DldWj2ZtNu1ZMqC04F+HyrIMMvjZFy25hv/4pbm3RDrCKAngLUgA57PhDfr4vIrxvgXKqxmPVlOYMAxRqNC99LZ2KxQgLQHU6k6UAVzK1hWCnyvVPlZxwRh3nuc2cKdEH5olDyuXgbL5EDb8175H8MlWMzBik9kC/hHUYDhkK/Z8kRKiDvWCh7fA0P7lzqzAjc78w402reEfNT4zNBfgVOK5E+WlMgTJ8tEwVH8rvKbny0jeEm5obTUWRAxj64LaxvhCs7L2yQAX2LxNWCTCXzF0Q14ZEDz6PKWFFYZB2SV14JyBZGdHc38j5DuDmRFPPXPBu0GGyYCrRPnBSnCXX2f4Z9GCGm6EW7LGcwe2Ij6dqG8BdyM0RYbP+HhZ75wRniTqEPAQp4fUGII7Q+4TvJVHuM8IS/RuwTKzXqNpNnRtxZLR7hdMdHXADkULOX0ji8/Y408ywTWWht4ivI1PIAyPMHlDVm9FcIOyjUxb9sxqX4P+kLTPseejrpnaWdJeAOHDviXjscsnGXVeiWutbkQYnhjn9EY8HJSXQDnSG9DqGhIgxGasd7jBgXw2tQZ/jcxTOyPks+6S1bYFXN/zYrOZTK+eciRwycirXAH3OQAyfi6EL0y35s/hfoimRP/xmblyQTLPj+iP068wPBC9Z/zgYFG9Ie+hYfy3CwXG0P41CLsGnLdMPK/pXmyWXcDdS5MGv8Dw9M8M7k6IBW8eAK4W+bW6JWF48rtK8IeWadw2rcT1p/sB8bKGNa5B7/dZ2qDvEJN/iw7sPc8TaVPDykqOqsmEG3xKCGK5A/9HaS5uBV3XdL33lFPRNT9Uw+UL2ejiYcCxSmVvZQMIf+S5DzwnyBriIzWRfGsC2jwrEda4rQD5/sCxyg2A0k7lvAK0C/HLzi5cbd7B/YChNf4bxmNnTqudUJmEoX377BA8OjILw48dGHrLzSvKkQ2yZJ1tPnMWdG+N/uh8zhs/vvNiqQbuvzkbE6r09OToMDD03/2s0AcaWofbTgh7jH/Ha1cdnKL7fQ8lDJ1f4Mnf54R/iFpt0+p3IsSPV2VtCtxZjhpP/H5pFkWEtysCrW5M6IZ1gaEN/nuAJgn+zU6ujVcZavRrZR9gdoP0T3VHesi34dEtj8zQXbPbJ9XFyX1Esp7ouVZ9fJ1W4S9+f1PdF7mvOdORTc/4sL4B0YBZqLyTOzE6qQ4wk34++r8TPBQ8SWSLjEZp+muDujmoHywZTT1ROsjvxz+a/gdDw2AYpKyXMQAAAABJRU5ErkJggg==";
    let mut page = Page::new(
        "GudLink".to_string(),
        "mygudlink".to_string(),
        logo.to_string(),
    );
    let mut group = Group::new("LINKS".to_string());
    group.add_element(PageElement::Folder(Folder::new(
        "Learn more about us".to_string(),
        "".to_string(),
    )));
    group.add_element(PageElement::Folder(Folder::new(
        "Our first case".to_string(),
        "".to_string(),
    )));
    group.add_element(PageElement::Link(Link::new(
        "Subscribe for our closed beta".to_string(),
        "#".to_string(),
        LinkType::PAGE,
    )));

    page.add_group(group);
    page.add_media(Media::new(MediaType::MEDIUM, "#".to_string()));
    page.add_media(Media::new(MediaType::LINKEDIN, "#".to_string()));
    page.add_media(Media::new(MediaType::TWITTER, "#".to_string()));
    page.add_media(Media::new(MediaType::INSTAGRAM, "#".to_string()));
    page.save().await?;

    Ok(())
}

pub async fn test_find_entities() -> Result<()> {
    println!("pages: {:#?}", Page::find(doc!{}).await?);
    println!("pages not found: {:#?}", Page::find(doc!{"teste": 2}).await?);
    println!("groups: {:#?}", Group::find(doc!{}).await?);
    println!("groups not found: {:#?}", Group::find(doc!{"teste": 2}).await?);
    Ok(())
}
pub fn static_files() -> fs::Files {
    fs::Files::new("/", "./build")
}

pub async fn run(address: &str) -> Result<(),std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .configure(libs::routes::config)
            .service(static_files())
    })
    .bind(address)?
    .run()
    .await
}
