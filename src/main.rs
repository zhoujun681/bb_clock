use chrono::{Local, DateTime};
use slint::{LogicalPosition,Brush};




slint::include_modules!();
fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    // ui.on_toggle_switch_toggled(||{});
    let timer_handle = ui_handle.clone(); // 克隆ui_handle给timer闭包
    ui.on_move_window(move |offset_x, offset_y|{
        // 在闭包里使用弱引用升级获取UI句柄
        if let Some(main) = ui_handle.upgrade() {
            let logical_pos = main.window().position().to_logical(main.window().scale_factor());
            main.window().set_position(LogicalPosition::new(logical_pos.x + offset_x, logical_pos.y + offset_y));
        }
    });
    

  // 创建一个Timer对象，设置回调来更新UI
  let timer = slint::Timer::default();


    let timer = timer.start(slint::TimerMode::Repeated, std::time::Duration::from_secs(1), move || {
        if let Some(ui) = timer_handle.clone().upgrade() {
            if(ui.get_toggle_switch_sec_checked()){
                ui.set_times(get_time("%Y-%m-%d %H:%M:%S").into());
            }else {
                ui.set_times(get_time("%Y-%m-%d %H:%M").into());
            }
            if(ui.get_toggle_switch_on_top_checked()){
                ui.set_on_top(true);
            }else {
                ui.set_on_top(false);
            }
            if(ui.get_toggle_switch_transparent_checked()){
                ui.set_my_background(Brush::SolidColor(slint::Color::from_argb_f32(0.0, 0.0, 0.0, 0.0)));
            }else {
                ui.set_my_background(Brush::SolidColor(slint::Color::from_argb_f32(0.5, 0.0, 0.0, 0.0)));
            }
      
        }
    });

   
    ui.run()
}

fn get_time(fmt:&str) -> String {
    let now: DateTime<Local> = Local::now();
    now.format(fmt).to_string()
}
