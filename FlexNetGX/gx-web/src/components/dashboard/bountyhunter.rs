// FlexNetGX/gx-web/src/components/dashboard/bountyhunter.rs
use yew::prelude::*;
use crate::components::gx-mobile::{Card, Tabs, Button, Modal};
use crate::components::bounty::bountyResponse;
use crate::services::bountyhunter::bountyhunterService;
use crate::types::{bounty, bountyhunterActivity, Badge, Task};

pub struct bountyhunterDashboard {
    link: ComponentLink<Self>,
    service: bountyhunterService,
    available_bounties: Vec<bounty>,
    completed_bounties: Vec<bounty>,
    current_activity: Option<bountyhunterActivity>,
    earned_badges: Vec<Badge>,
    assigned_tasks: Vec<Task>,
    loading: bool,
    active_bounty: Option<bounty>,
    show_bounty_modal: bool,
    error: Option<String>,
}

pub enum Msg {
    FetchData,
    DataReceived {
        bounties: Vec<bounty>,
        completed: Vec<bounty>,
        badges: Vec<Badge>,
        tasks: Vec<Task>,
    },
    Startbounty(bounty),
    Submitbounty(bounty, Vec<String>),
    CompleteTask(String),
    CloseModal,
    UpdateActivity(bountyhunterActivity),
    Error(String),
}

impl Component for bountyhunterDashboard {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut dashboard = Self {
            link,
            service: bountyhunterService::new(),
            available_bounties: vec![],
            completed_bounties: vec![],
            current_activity: None,
            earned_badges: vec![],
            assigned_tasks: vec![],
            loading: true,
            active_bounty: None,
            show_bounty_modal: false,
            error: None,
        };
        
        dashboard.link.send_message(Msg::FetchData);
        dashboard
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchData => {
                self.loading = true;
                let link = self.link.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match bountyhunterService::fetch_bountyhunter_data().await {
                        Ok(data) => link.send_message(Msg::DataReceived {
                            bounties: data.available_bounties,
                            completed: data.completed_bounties,
                            badges: data.earned_badges,
                            tasks: data.assigned_tasks,
                        }),
                        Err(e) => link.send_message(Msg::Error(e.to_string())),
                    }
                });
                false
            }
            Msg::DataReceived { bounties, completed, badges, tasks } => {
                self.available_bounties = bounties;
                self.completed_bounties = completed;
                self.earned_badges = badges;
                self.assigned_tasks = tasks;
                self.loading = false;
                true
            }
            Msg::Startbounty(bounty) => {
                self.active_bounty = Some(bounty);
                self.show_bounty_modal = true;
                true
            }
            Msg::Submitbounty(bounty, responses) => {
                let link = self.link.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match bountyhunterService::submit_bounty_response(bounty.id, responses).await {
                        Ok(_) => link.send_message(Msg::FetchData),
                        Err(e) => link.send_message(Msg::Error(e.to_string())),
                    }
                });
                self.show_bounty_modal = false;
                true
            }
            // Handle other messages...
            _ => false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="flex flex-col h-full bg-gray-100">
                { self.view_header() }
                <div class="flex-grow p-6">
                    <Tabs>
                        <div label="Dashboard">
                            { self.view_dashboard_content() }
                        </div>
                        <div label="bounties">
                            { self.view_bounties() }
                        </div>
                        <div label="Tasks">
                            { self.view_tasks() }
                        </div>
                        <div label="Badges">
                            { self.view_badges() }
                        </div>
                    </Tabs>
                </div>
                { self.view_bounty_modal() }
            </div>
        }
    }
}

impl bountyhunterDashboard {
    fn view_header(&self) -> Html {
        html! {
            <header class="bg-white shadow-sm p-4">
                <div class="flex justify-between items-center">
                    <h1 class="text-2xl font-bold text-gray-800">
                        {"bountyhunter Dashboard"}
                    </h1>
                    { self.view_activity_status() }
                </div>
            </header>
        }
    }

    fn view_dashboard_content(&self) -> Html {
        html! {
            <div class="grid grid-cols-12 gap-6">
                { self.view_stats_overview() }
                { self.view_recent_activity() }
                { self.view_upcoming_tasks() }
            </div>
        }
    }

    fn view_stats_overview(&self) -> Html {
        html! {
            <div class="col-span-12 grid grid-cols-4 gap-4">
                <Card>
                    <div class="p-4">
                        <h3 class="text-lg font-semibold text-gray-600">
                            {"bounties Completed"}
                        </h3>
                        <p class="text-3xl font-bold">
                            { self.completed_bounties.len() }
                        </p>
                    </div>
                </Card>
                <Card>
                    <div class="p-4">
                        <h3 class="text-lg font-semibold text-gray-600">
                            {"Available bounties"}
                        </h3>
                        <p class="text-3xl font-bold">
                            { self.available_bounties.len() }
                        </p>
                    </div>
                </Card>
                <Card>
                    <div class="p-4">
                        <h3 class="text-lg font-semibold text-gray-600">
                            {"Badges Earned"}
                        </h3>
                        <p class="text-3xl font-bold">
                            { self.earned_badges.len() }
                        </p>
                    </div>
                </Card>
                <Card>
                    <div class="p-4">
                        <h3 class="text-lg font-semibold text-gray-600">
                            {"Active Tasks"}
                        </h3>
                        <p class="text-3xl font-bold">
                            { self.assigned_tasks.len() }
                        </p>
                    </div>
                </Card>
            </div>
        }
    }

    fn view_bounty_modal(&self) -> Html {
        if let Some(bounty) = &self.active_bounty {
            html! {
                <Modal
                    show=self.show_bounty_modal
                    onclose=self.link.callback(|_| Msg::CloseModal)
                    title=bounty.title.clone()
                >
                    <bountyResponse
                        bounty=bounty.clone()
                        onsubmit=self.link.callback(move |responses| 
                            Msg::Submitbounty(bounty.clone(), responses)
                        )
                    />
                </Modal>
            }
        } else {
            html! {}
        }
    }
}