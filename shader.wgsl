@vertex // Атрибут указывает что функция относится к вершинному шейдеру.
// Функция принимает индекс (порядковый номер) вершины и возвращает её положение на экране.
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    // Здесь применим небольшой математический трюк.
    // Чтобы не подгружать точки треугольника извне, рассчитаем их координаты
    // исходя из порядкового номера.
    let x = f32(i32(in_vertex_index) - 1); // f32() и i32() - преведение типов.
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    // Легко убедиться, что индексы [0, 1, 2]
    // дадут вершины [(-1,-1), (0,1), (1,-1)].

    // Возвращаем позицию вершины
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment // Атрибут указывает что функция относится к фрагментному шейдеру.
// Функция не принимает ничего, а возвращает цвет фрагмента в формате RGBA.
fn fs_main() -> @location(0) vec4<f32> {
    // Возвращаем красный цвет для каждого фрагмента нашего треугольника.
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}