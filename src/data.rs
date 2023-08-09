#![windows_subsystem = "windows"]

use std::time::Duration;

use chrono::Utc;
use chrono_tz::{America::New_York, Asia::Seoul};
use druid::{widget::Controller, Data, Env, Event, EventCtx, Lens, Widget};

#[derive(Clone, Data, Lens)]
pub struct TimeState {
    pub(crate) time: String,
}

#[derive(Clone, Data)]
pub struct TimeController;

impl<W: Widget<TimeState>> Controller<TimeState, W> for TimeController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut TimeState,
        env: &Env,
    ) {
        match event {
            Event::WindowConnected => {
                let timer_id = ctx.request_timer(Duration::from_secs(1));
                ctx.set_handled();
            }
            Event::Timer(timer_id) => {
                data.time = data.get_time();
                ctx.request_paint();
                let timer_id = ctx.request_timer(Duration::from_secs(1));
                ctx.set_handled();
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
        ctx.window().set_always_on_top(true);
        ctx.window().show_titlebar(false);
        // ctx.window().handle_titlebar(true);
    }
}

impl TimeState {
    fn get_time(&mut self) -> String {
        let raw_ny_time = Utc::now().with_timezone(&New_York).to_string();
        let raw_kr_time = Utc::now().with_timezone(&Seoul).to_string();
        return format!(
            "NewYork_Time: \n {:.19}
            Korea Time: \n {:.19}",
            raw_ny_time, raw_kr_time
        );
    }
}
