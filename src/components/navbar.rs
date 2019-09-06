use failure::Error;
use serde_derive::{Deserialize, Serialize};
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::services::Task;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

type AsBinary = bool;

pub enum Format {
    Json,
}

pub struct AppBarHeader {
    fetch_service: FetchService,
    ws_service: WebSocketService,
    link: ComponentLink<AppBarHeader>,
    fetching: bool,
    data: Option<u32>,
    ft: Option<FetchTask>,
    ws: Option<WebSocketTask>,
}

pub enum WsAction {
    Connect,
    SendData(AsBinary),
    Disconnect,
    Lost,
}

pub enum Msg {
    FetchData(Format, AsBinary),
    WsAction(WsAction),
    FetchReady(Result<DataFromFile, Error>),
    WsReady(Result<WsResponse, Error>),
    Ignore,
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Msg::WsAction(action)
    }
}

/// This type is used to parse data from `./static/data.json` file and
/// have to correspond the data layout from that file.
#[derive(Deserialize, Debug)]
pub struct DataFromFile {
    value: u32,
}

/// This type is used as a request which sent to websocket connection.
#[derive(Serialize, Debug)]
struct WsRequest {
    value: u32,
}

/// This type is an expected response from a websocket connection.
#[derive(Deserialize, Debug)]
pub struct WsResponse {
    value: u32,
}

impl Component for AppBarHeader {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        AppBarHeader {
            fetch_service: FetchService::new(),
            ws_service: WebSocketService::new(),
            link,
            fetching: false,
            data: None,
            ft: None,
            ws: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchData(format, binary) => {
                stdweb::web::alert("Fetching Data");
                self.fetching = true;
                let task = match format {
                    Format::Json => {
                        let callback = self.link.send_back(
                            move |response: Response<Json<Result<DataFromFile, Error>>>| {
                                let (meta, Json(data)) = response.into_parts();
                                println!("META: {:?}, {:?}", meta, data);
                                if meta.status.is_success() {
                                    Msg::FetchReady(data)
                                } else {
                                    Msg::Ignore // FIXME: Handle this error accordingly.
                                }
                            },
                        );
                        let request = Request::get("/data/data.json").body(Nothing).unwrap();
                        if binary {
                            self.fetch_service.fetch_binary(request, callback)
                        } else {
                            self.fetch_service.fetch(request, callback)
                        }
                    }
                };
                self.ft = Some(task);
            }
            Msg::WsAction(action) => match action {
                WsAction::Connect => {
                    stdweb::web::alert("Msg::WsAction(action: WsAction) => match action");

                    stdweb::web::alert("WSAction::Connect");

                    let callback = self.link.send_back(|Json(data)| Msg::WsReady(data));
                    let notification = self.link.send_back(|status| match status {
                        WebSocketStatus::Opened => {
                            stdweb::web::alert("WebSocketStatus::Opened");

                            Msg::Ignore
                        }
                        WebSocketStatus::Closed | WebSocketStatus::Error => {
                            stdweb::web::alert("Closed or Error");
                            WsAction::Lost.into()
                        }
                    });
                    let task =
                        self.ws_service
                            .connect("ws://127.0.0.1:8001/", callback, notification);
                    self.ws = Some(task);
                }
                WsAction::SendData(binary) => {
                    stdweb::web::alert("Send Data");
                    let request = WsRequest { value: 321 };
                    if binary {
                        self.ws.as_mut().unwrap().send_binary(Json(&request));
                    } else {
                        self.ws.as_mut().unwrap().send(Json(&request));
                    }
                }
                WsAction::Disconnect => {
                    stdweb::web::alert("Disconnect");
                    self.ws.take().unwrap().cancel();
                }
                WsAction::Lost => {
                    stdweb::web::alert("Lost");
                    self.ws = None;
                }
            },
            Msg::FetchReady(response) => {
                stdweb::web::alert("Fetch Ready");
                self.fetching = false;
                self.data = response.map(|data| data.value).ok();
            }
            Msg::WsReady(response) => {
                stdweb::web::alert("WS Ready");
                self.data = response.map(|data| data.value).ok();
            }
            Msg::Ignore => {
                stdweb::web::alert("Ignore");
                return false;
            }
        }
        true
    }
}

impl Renderable<AppBarHeader> for AppBarHeader {
    fn view(&self) -> Html<Self> {
        html! {
        <nav class="grey lighten-5" style="border: 3px solid #15E900;">
          <div class="nav-wrapper">
              <a href="#!" class="brand-logo left">{ self.view_logo() }</a>
            <a href="#!" class="brand-logo left" style="color: #6800F7;font-size: 38px;padding-left: 69px;">{ "Valknut Engine" }</a>

            <ul class="right ">
              <li><a href="#" class="account"><i class="fa fa-comments" style="padding-right: 5px;"></i>{ "Chat" }</a></li>
              
              <li><a href="#" class="account"><i class="material-icons left">{ "account_circle" }</i>{ "Account" }</a></li>
            </ul>
          </div>
        </nav>


              }
    }
}

impl AppBarHeader {
    fn view_logo(&self) -> Html<AppBarHeader> {
        html! {
                                    <img src="assets/logo.svg" class="logo" />


        }
    }

    fn render_account(&self) -> Html<AppBarHeader> {
        html! {
          <ul id="dropdown1" class="dropdown-content">
            <li><a href="#!">{ "Manage Campaigns" }</a></li>
            <li><a href="#!">{ "Launch Campaign" }</a></li>
            <li><a href="#!">{ "Campaign Laws" }</a></li>
          </ul>
        }
    }

    fn render_settings(&self) -> Html<AppBarHeader> {
        html! {
            <ul id="campaign_dropdown" class="dropdown-content">
                <li><a href="#">{ "text" }</a></li>
                <li class="divider"></li>
            </ul>
        }
    }
}
