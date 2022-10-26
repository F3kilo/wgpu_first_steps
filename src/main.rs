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

    // Включим код шейдера в исполняемый файл с помощью макроса include_str.
    let shader_code = include_str!("../shader.wgsl");

    // Создадим объект шейдера из его кода.
    let descriptor = wgpu::ShaderModuleDescriptor {
        label: None, // Метку для отладки оставим не заданной.
        source: wgpu::ShaderSource::Wgsl(shader_code.into()),
    };
    let shader_module = device.create_shader_module(descriptor);

    // Определим, какой формат изображения лучше всего подходит для выбранного адаптера.
    let surface_format = surface.get_supported_formats(&adapter)[0];


    // Зададим параметры целевого изображения. В нашем случае - поверхности в окне.
    let color_target = wgpu::ColorTargetState {
        // Параметры цели для отрисовки.
        format: surface_format,         // Формат целевого изображения.
        blend: None,                    // Смешение цветов не используем.
        write_mask: Default::default(), // Пишем во все каналы RGBA.
    };

    // Поместим параметры целевого изображения в массив.
    let color_targets = [Some(color_target)];

    // Параметры графического конвейера оставим, в основном, по умолчанию.
    let descriptor = wgpu::RenderPipelineDescriptor {
        label: None,                   // Метку для отладки оставим не заданной.
        primitive: Default::default(), // Создание фигур из вершин - по умолчанию.
        vertex: wgpu::VertexState {
            // Параметры вершинного шейдера.
            buffers: &[],           // Буффер с данными о вершинах не используется.
            module: &shader_module, // Идентификатор нашего шейдера.
            entry_point: "vs_main", // Имя функции, которая будет вызываться для вершин.
        },
        fragment: Some(wgpu::FragmentState {
            // Параметры фрагментного шейдера.
            targets: &color_targets, // Параметры целевого изображения.
            module: &shader_module,  // Идентификатор нашего шейдера.
            entry_point: "fs_main",  // Имя функции, которая будет вызываться для фрагментов.
        }),
        layout: None, // Разметку для передачи внешних данных в шейдер не используем.
        depth_stencil: None, // Тест глубины нам не нужен.
        multisample: Default::default(), // Multisample по умолчанию отключен.
        multiview: None, // Отображение будет происходить только в одно изображение.
    };
    let pipeline = device.create_render_pipeline(&descriptor);

    // Настроим поверхность в соттвитствии с параметрами окна:
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // Будем использовать surface для рендера.
        format: surface_format,                        // Формат, который мы выбрали ранее.
        width: window_size.width,                      // Ширина окна.
        height: window_size.height,                    // Высота окна.
        present_mode: wgpu::PresentMode::Mailbox,      // Алгоритм вывода кадров на экран.
        alpha_mode: wgpu::CompositeAlphaMode::Auto,    // Использование альфа канала.
    };
    surface.configure(&device, &config);

    // Запустим цикл обработки событий, передав в него замыкание,
    // которое будет выполнятся на кождой итерации цикла.
    event_loop.run(move |event, _, control_flow| {
        // Будем попадать в тело цикла только при появлении события ОС.
        *control_flow = ControlFlow::Wait;

        match event {
            // Если обработаны все накопившиеся события - перерисовываем содержимое окна.
            Event::MainEventsCleared => {
                // Получаем следующий кадр.
                let frame = surface.get_current_texture().unwrap();

                // Создаём View для изображения этого кадра.
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                // Создаём CommandEncoder.
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                // Новая область видимости нужна, чтобы компилятор видел,
                // что RenderPass живёт не дольше, чем CommandEncoder.
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None, // Метку для отладки оставим не заданной.
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,          // Цель для отрисовки.
                            resolve_target: None, // Используется для мультисэмплинга.
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLUE), // Очищаем кадр синим цветом.
                                store: true, // Сохраняем содержимое после завершения данного RenderPass.
                            },
                        })],
                        depth_stencil_attachment: None, // Буфер глубины не используем.
                    });

                    // Задаём графический конвейер.
                    // Все последующие операции рендеринга будут исполняться на нём.
                    rpass.set_pipeline(&pipeline);

                    // Отрисоваваем один объект с тремя вершинами.
                    rpass.draw(0..3, 0..1);
                }

                // Сохраняем в буфер команды, записанные в CommandEncoder.
                let command_buffer = encoder.finish();

                // Передаём буфер в очередь команд устройства.
                queue.submit(Some(command_buffer));

                // Отображаем на экране отрендеренный кадр.
                frame.present();
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
