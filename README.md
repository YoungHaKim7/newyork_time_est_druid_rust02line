# newyork_time_est_druid_rust02line

- My Settings

```rust
        child.event(ctx, event, data, env);
        ctx.window().set_always_on_top(true);
        ctx.window().show_titlebar(false);
        ctx.window().handle_titlebar(true);
        ctx.window().set_position((1760., 0.0));

..//
fn build_root_widget() -> impl Widget<TimeState> {
    let display = Label::new(|data: &String, _env: &_| data.clone())
        .with_text_size(16.0)
        .lens(TimeState::time)
        .padding(5.0);


..//

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("NewYork_Time: EST!")
        .window_size((160.0, 90.0))
        .resizable(false);


```

# Update(230808)

```rust
45    ctx.window().set_always_on_top(true);
46    ctx.window().show_titlebar(false);
47    ctx.window().handle_titlebar(true);
```


![02](https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/c16924c9-d20c-426b-8bc6-6f54f353e753)


# Result

![01](https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/f628d962-fa53-446c-bc57-991adcfa23db)

