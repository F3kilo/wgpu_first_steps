use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() {
    // Создадим цикл обработки событий.
    let event_loop = EventLoop::new();

    let window_size: PhysicalSize<u32> = (800, 600).into();

    // Создадим окно, задав его параметры.
    let window = WindowBuilder::new()
        .with_fullscreen(None)
        .with_inner_size(window_size)
        .with_title("wgpu first steps")
        .build(&event_loop)
        .unwrap();

    // Создание экземпляра графического API.
    let instance = wgpu::Instance::new(wgpu::Backends::all());

    // Создаём поверхность для отображения.
    let surface = unsafe { instance.create_surface(&window) };

    // Задаём желаемые параметры графического устройства
    let adapter_options = wgpu::RequestAdapterOptions {
        compatible_surface: Some(&surface),
        ..Default::default()
    };
    // Запрос адаптера - асинхронная операция.
    let adapter_future = instance.request_adapter(&adapter_options);
    // Дождёмся её завершения с помощью pollster.
    let adapter = pollster::block_on(adapter_future).unwrap();
    // Выведем в терминал название выбранного адаптера.
    println!("Selected adapter: {}", adapter.get_info().name);

    // Зададим параметры устройства по умолчанию.
    let device_descriptor = wgpu::DeviceDescriptor::default();
    // Запрос устройства - асинхронная операция.
    let device_future = adapter.request_device(&device_descriptor, None);
    // Получаем логическое устройство и очередь задач для него.
    let (device, queue) = pollster::block_on(device_future).unwrap();

    // Запустим цикл обработки событий, передав в него замыкание,
    // которое будет выполнятся на кождой итерации цикла.
    event_loop.run(move |event, _, control_flow| {
        // Будем попадать в тело цикла только при появлении события ОС.
        *control_flow = ControlFlow::Wait;

        match event {
            // Если обработаны все накопившиеся события - перерисовываем содержимое окна.
            Event::MainEventsCleared => {
                // todo: код рендера
            }

            // Если было запрошено закрытие окна, завершаем цикл.
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }

            // Остальные события нам не интересны.
            _ => {}
        }
    })
}
