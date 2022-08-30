use gloo_net::http::Request;
use yew::prelude::*;

mod vid;
use vid::{Video, VideoDetails, VideosList};

// Extracted the api request to its own function
// Box<dyn std::error::Error> means anything that looks like an error is okay
async fn get_videos() -> Result<Vec<Video>, Box<dyn std::error::Error>> {
    let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
        .send()
        .await?
        .json()
        .await?;
    Ok(fetched_videos)
}

// Picked up this pattern from an elm conf video. Anytime we have a component
// that gets data from an API request, we should store that data inside an enum
// that represents the stages of that request. You'll see how this is used below.
// In a real codebase, I would make this generic and use it anytime I made an api
// request from inside a component.
#[derive(Clone)]
pub enum RequestedVideos {
    Loading,
    Error,
    Success(Vec<Video>),
}

pub struct App {
    videos: RequestedVideos,
    selected_video: Option<Video>,
}

// I like having a Msg enum because it serves as one location to see everything thats "dynamic"
// about a component. This component changes based on two events, getting the videos from the
// api and selecting a video by clicking on the title.
pub enum AppMsg {
    SetVideos(RequestedVideos),
    SelectVideo(Video),
}

impl Component for App {
    type Message = AppMsg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // When the component is created, we haven't made the api request yet so
        // the videos are set to "loading" and there is no selected video
        Self {
            videos: RequestedVideos::Loading,
            selected_video: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Since we know the state of the request, we actually get the opportunity to display custom loading and
        // error markup (like a loading wheel) instead of having awkward blank space on slow connections.
        match self.videos.clone() {
            RequestedVideos::Loading => {
                html! {
                    <h1>{"Videos Loading"}</h1>
                }
            }
            RequestedVideos::Error => {
                html! {
                    <h1>{"Error loading videos!"}</h1>
                }
            }
            RequestedVideos::Success(vids) => {
                html! {
                    <div>
                        <h1>{ "RustConf Explorer" }</h1>
                        <div>
                            <h3>{"Videos to watch"}</h3>
                            <VideosList videos={vids} on_click={ctx.link().callback(|video: Video| AppMsg::SelectVideo(video.clone()))} />
                        </div>

                        // I find this clearer than the { for details } trick with option
                        if self.selected_video.is_some() {
                            <VideoDetails video={self.selected_video.clone().unwrap()} />
                        }

                    </div>
                }
            }
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::SetVideos(res) => self.videos = res,
            AppMsg::SelectVideo(video) => self.selected_video = Some(video),
        }
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // On the first render of the component, we make the api request.
        if first_render {
            let link = ctx.link().clone();
            wasm_bindgen_futures::spawn_local(async move {
                match get_videos().await {
                    Ok(vids) => {
                        link.send_message(AppMsg::SetVideos(RequestedVideos::Success(vids)))
                    }
                    Err(_) => link.send_message(AppMsg::SetVideos(RequestedVideos::Error)),
                }
            });
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
