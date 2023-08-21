use yew::prelude::*;

use crate::video::Video;

#[derive(Properties, PartialEq)]
pub struct VideoDetailsProps {
    pub video: Video,
}

#[function_component(VideoDetails)]
pub fn video_details(VideoDetailsProps { video }: &VideoDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}