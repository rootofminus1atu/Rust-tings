mod components {
    pub mod video_list;
    pub mod video_details;
}
pub mod video;

use components::{
    video_list::VideoList,
    video_details::VideoDetails
};
use video::Video;

use yew::prelude::*;
use gloo_net::http::Request;



pub fn get_videos() -> Vec<Video> {
    vec![
        Video {
            id: 1,
            title: "Building and breaking things".to_string(),
            speaker: "Bingleshoe The 3rd".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 2,
            title: "The development process".to_string(),
            speaker: "Jane Doe".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".to_string(),
            speaker: "Matt Miller".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 4,
            title: "Mouseless development".to_string(),
            speaker: "Tom Jerry".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
    ]
}





#[function_component(App)]
fn app() -> Html {
    let selected_video = use_state(|| None);

    let videos = use_state(|| vec![]);



    {
        let videos = videos.clone();
        use_effect_with_deps(move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                videos.set(fetched_videos);
            });
        }, ());
    }



    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video));
        })
    };

    let details = selected_video.as_ref().map(|video| html! {
        <VideoDetails video={video.clone()} />
    });

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideoList videos={(*videos).clone()} on_click={on_video_select} />
            </div>
            { details }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
