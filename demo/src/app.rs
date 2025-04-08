use yew::prelude::*;
use web_sys::{wasm_bindgen::JsCast, Element, HtmlElement, DomRect};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use js_sys::Promise;
use freedraw::{get_stroke, get_svg_path_from_stroke, StrokeOptions, InputPoint, TaperOptions, TaperType};

#[derive(Properties, PartialEq)]
pub struct SvgExampleProps {
    pub title: String,
    pub src: String,
    pub filename: String,
}

#[function_component(SvgExample)]
fn svg_example(props: &SvgExampleProps) -> Html {
    html! {
        <div class="example">
            <h3>{ &props.title }</h3>
            <div class="svg-container">
                <img src={props.src.clone()} alt={props.title.clone()} loading="lazy" />
            </div>
            <p>{ &props.filename }</p>
        </div>
    }
}

#[derive(Clone, PartialEq)]
enum StrokeStyle {
    Default,
    DefaultWithStroke,
    Tapered,
    TaperedWithStroke,
    Thin,
    ThinWithStroke,
    Thick,
    ThickWithStroke
}

impl StrokeStyle {
    fn get_options(&self) -> (StrokeOptions, bool, bool) {
        // Returns (options, is_stroke_only, is_tapered)
        match self {
            StrokeStyle::Default => (
                StrokeOptions {
                    size: Some(16.0),
                    thinning: Some(0.5),
                    smoothing: Some(0.5),
                    streamline: Some(0.5),
                    simulate_pressure: Some(false),
                    closed: Some(false),
                    ..Default::default()
                },
                false,
                false
            ),
            StrokeStyle::DefaultWithStroke => (
                StrokeOptions {
                    size: Some(16.0),
                    thinning: Some(0.5),
                    smoothing: Some(0.5),
                    streamline: Some(0.5),
                    simulate_pressure: Some(false),
                    closed: Some(false),
                    ..Default::default()
                },
                true,
                false
            ),
            StrokeStyle::Tapered => (
                StrokeOptions {
                    size: Some(16.0),
                    thinning: Some(0.5),
                    smoothing: Some(0.5),
                    streamline: Some(0.5),
                    simulate_pressure: Some(false),
                    start: Some(TaperOptions {
                        taper: Some(TaperType::Bool(true)),
                        ..Default::default()
                    }),
                    end: Some(TaperOptions {
                        taper: Some(TaperType::Bool(true)),
                        ..Default::default()
                    }),
                    closed: Some(false),
                    ..Default::default()
                },
                false,
                true
            ),
            StrokeStyle::TaperedWithStroke => (
                StrokeOptions {
                    size: Some(16.0),
                    thinning: Some(0.5),
                    smoothing: Some(0.5),
                    streamline: Some(0.5),
                    simulate_pressure: Some(false),
                    start: Some(TaperOptions {
                        taper: Some(TaperType::Bool(true)),
                        ..Default::default()
                    }),
                    end: Some(TaperOptions {
                        taper: Some(TaperType::Bool(true)),
                        ..Default::default()
                    }),
                    closed: Some(false),
                    ..Default::default()
                },
                true,
                true
            ),
            StrokeStyle::Thin => (
                StrokeOptions {
                    size: Some(10.0),
                    thinning: Some(0.75),
                    smoothing: Some(0.5),
                    streamline: Some(0.5),
                    simulate_pressure: Some(false),
                    closed: Some(false),
                    ..Default::default()
                },
                false,
                false
            ),
            StrokeStyle::ThinWithStroke => (
                StrokeOptions {
                    size: Some(10.0),
                    thinning: Some(0.75),
                    smoothing: Some(0.5),
                    streamline: Some(0.5),
                    simulate_pressure: Some(false),
                    closed: Some(false),
                    ..Default::default()
                },
                true,
                false
            ),
            StrokeStyle::Thick => (
                StrokeOptions {
                    size: Some(20.0),
                    thinning: Some(0.25),
                    smoothing: Some(0.5),
                    streamline: Some(0.5),
                    simulate_pressure: Some(false),
                    closed: Some(false),
                    ..Default::default()
                },
                false,
                false
            ),
            StrokeStyle::ThickWithStroke => (
                StrokeOptions {
                    size: Some(20.0),
                    thinning: Some(0.25),
                    smoothing: Some(0.5),
                    streamline: Some(0.5),
                    simulate_pressure: Some(false),
                    closed: Some(false),
                    ..Default::default()
                },
                true,
                false
            )
        }
    }
    
    fn to_str(&self) -> &'static str {
        match self {
            StrokeStyle::Default => "Default",
            StrokeStyle::DefaultWithStroke => "Default with Stroke",
            StrokeStyle::Tapered => "Tapered",
            StrokeStyle::TaperedWithStroke => "Tapered with Stroke",
            StrokeStyle::Thin => "Thin",
            StrokeStyle::ThinWithStroke => "Thin with Stroke",
            StrokeStyle::Thick => "Thick",
            StrokeStyle::ThickWithStroke => "Thick with Stroke"
        }
    }
}

// Added new DrawingBoard component for interactive drawing
#[function_component(DrawingBoard)]
fn drawing_board() -> Html {
    use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, window};
    use freedraw::{get_stroke, get_svg_path_from_stroke, StrokeOptions, InputPoint};
    
    let canvas_ref = use_node_ref();
    // Instead of a simple array of points, we'll track multiple strokes
    let strokes = use_state(|| Vec::<Vec<(f64, f64)>>::new());
    let current_stroke = use_state(|| Vec::<(f64, f64)>::new());
    let drawing = use_state(|| false);
    let svg_paths = use_state(|| Vec::<(String, bool)>::new());
    let copy_message = use_state(|| String::new());
    let show_trace = use_state(|| false);
    let stroke_style = use_state(|| StrokeStyle::Default);

    // Helper function to get canvas coordinates
    let get_canvas_coordinates = |canvas: &HtmlCanvasElement, client_x: i32, client_y: i32| -> (f64, f64) {
        let rect = canvas.get_bounding_client_rect();
        let scale_x = canvas.width() as f64 / rect.width();
        let scale_y = canvas.height() as f64 / rect.height();
        
        let x = (client_x as f64 - rect.left()) * scale_x;
        let y = (client_y as f64 - rect.top()) * scale_y;
        (x, y)
    };

    // Helper function to update the SVG paths based on all strokes
    let update_svg_paths = {
        let strokes = strokes.clone();
        let current_stroke = current_stroke.clone();
        let svg_paths = svg_paths.clone();
        let stroke_style = stroke_style.clone();
        move || {
            let mut new_paths = Vec::new();
            let (_, is_stroke_only, _) = stroke_style.get_options();
            
            // Process all completed strokes
            for stroke_points in strokes.iter() {
                if stroke_points.len() > 0 {
                    let input_points: Vec<InputPoint> = stroke_points
                        .iter()
                        .map(|(x, y)| InputPoint::Array([*x, *y], None))
                        .collect();
                    
                    let (options, _, _) = stroke_style.get_options();
                    
                    let stroke = get_stroke(&input_points, &options);
                    if !stroke.is_empty() {
                        let path = get_svg_path_from_stroke(&stroke, false);
                        if !path.is_empty() {
                            new_paths.push((path, is_stroke_only));
                        }
                    }
                }
            }
            
            // Process the current stroke if it exists
            if current_stroke.len() > 0 {
                let input_points: Vec<InputPoint> = current_stroke
                    .iter()
                    .map(|(x, y)| InputPoint::Array([*x, *y], None))
                    .collect();
                
                let (options, _, _) = stroke_style.get_options();
                
                let stroke = get_stroke(&input_points, &options);
                if !stroke.is_empty() {
                    let path = get_svg_path_from_stroke(&stroke, false);
                    if !path.is_empty() {
                        new_paths.push((path, is_stroke_only));
                    }
                }
            }
            
            svg_paths.set(new_paths);
        }
    };

    let onmousedown = {
        let drawing = drawing.clone();
        let current_stroke = current_stroke.clone();
        let canvas_ref = canvas_ref.clone();
        let show_trace = show_trace.clone();
        Callback::from(move |e: web_sys::MouseEvent| {
            drawing.set(true);
            
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let (x, y) = get_canvas_coordinates(&canvas, e.client_x(), e.client_y());
                
                // Start a new current stroke
                let mut new_stroke = Vec::new();
                new_stroke.push((x, y));
                current_stroke.set(new_stroke);
                
                if *show_trace {
                    if let Ok(ctx_option) = canvas.get_context("2d") {
                        if let Some(ctx_js) = ctx_option {
                            if let Ok(ctx) = ctx_js.dyn_into::<CanvasRenderingContext2d>() {
                                ctx.begin_path();
                                ctx.set_stroke_style(&JsValue::from_str("red"));
                                ctx.set_line_width(2.0);
                                ctx.move_to(x, y);
                                ctx.stroke();
                            }
                        }
                    }
                }
            }
        })
    };

    let onmousemove = {
        let drawing = drawing.clone();
        let current_stroke = current_stroke.clone();
        let canvas_ref = canvas_ref.clone();
        let update_svg_paths = update_svg_paths.clone();
        let show_trace = show_trace.clone();
        Callback::from(move |e: web_sys::MouseEvent| {
            if *drawing {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    let (x, y) = get_canvas_coordinates(&canvas, e.client_x(), e.client_y());
                    
                    // Add to the current stroke
                    let mut new_stroke = (*current_stroke).clone();
                    new_stroke.push((x, y));
                    current_stroke.set(new_stroke);
                    
                    // Update SVG in real-time
                    update_svg_paths();
                    
                    if *show_trace {
                        if let Ok(ctx_option) = canvas.get_context("2d") {
                            if let Some(ctx_js) = ctx_option {
                                if let Ok(ctx) = ctx_js.dyn_into::<CanvasRenderingContext2d>() {
                                    ctx.set_stroke_style(&JsValue::from_str("red"));
                                    ctx.set_line_width(2.0);
                                    ctx.line_to(x, y);
                                    ctx.stroke();
                                }
                            }
                        }
                    }
                }
            }
        })
    };

    let onmouseup = {
        let drawing = drawing.clone();
        let strokes = strokes.clone();
        let current_stroke = current_stroke.clone();
        let update_svg_paths = update_svg_paths.clone();
        Callback::from(move |_| {
            drawing.set(false);
            
            // Add the current stroke to the completed strokes
            if !current_stroke.is_empty() {
                let mut new_strokes = (*strokes).clone();
                new_strokes.push((*current_stroke).clone());
                strokes.set(new_strokes);
                current_stroke.set(Vec::new());
                
                // Update SVG paths
                update_svg_paths();
            }
        })
    };

    let clear_drawing = {
        let strokes = strokes.clone();
        let current_stroke = current_stroke.clone();
        let svg_paths = svg_paths.clone();
        let canvas_ref = canvas_ref.clone();
        Callback::from(move |_: MouseEvent| {
            strokes.set(Vec::new());
            current_stroke.set(Vec::new());
            svg_paths.set(Vec::new());
            
            // Clear the canvas
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                if let Ok(ctx_option) = canvas.get_context("2d") {
                    if let Some(ctx_js) = ctx_option {
                        if let Ok(ctx) = ctx_js.dyn_into::<CanvasRenderingContext2d>() {
                            ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
                        }
                    }
                }
            }
        })
    };

    let toggle_trace = {
        let show_trace = show_trace.clone();
        let canvas_ref = canvas_ref.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            let new_value = input.checked();
            show_trace.set(new_value);
            
            // Clear the canvas when toggling off to hide existing traces
            if !new_value {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    if let Ok(ctx_option) = canvas.get_context("2d") {
                        if let Some(ctx_js) = ctx_option {
                            if let Ok(ctx) = ctx_js.dyn_into::<CanvasRenderingContext2d>() {
                                ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
                            }
                        }
                    }
                }
            }
        })
    };

    let copy_svg = {
        let svg_paths = svg_paths.clone();
        let copy_message = copy_message.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(window) = window() {
                let navigator = window.navigator();
                let clipboard = navigator.clipboard();
                
                let mut svg_content = String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1000 400">"#);
                
                // Add each path as a separate element
                for (path, is_stroke_only) in svg_paths.iter() {
                    svg_content.push_str(&format!(
                        r#"<path d="{}" fill="{}" stroke="white" stroke-width="{}" stroke-linejoin="round" stroke-linecap="round" />"#,
                        path,
                        if *is_stroke_only { "none" } else { "white" },
                        if *is_stroke_only { "2.5" } else { "1" }
                    ));
                }
                
                svg_content.push_str("</svg>");
                
                let window_clone = window.clone();
                
                // Create a closure that handles the Promise result
                let success_callback = {
                    let copy_message_clone = copy_message.clone();
                    let window_clone = window_clone.clone();
                    Closure::wrap(Box::new(move |_: JsValue| {
                        copy_message_clone.set("SVG copied to clipboard!".to_string());
                        
                        // Reset the message after 2 seconds
                        let copy_message_inner = copy_message_clone.clone();
                        let timeout_closure = Closure::wrap(Box::new(move || {
                            copy_message_inner.set("".to_string());
                        }) as Box<dyn FnMut()>);
                        
                        let _ = window_clone.set_timeout_with_callback_and_timeout_and_arguments_0(
                            timeout_closure.as_ref().unchecked_ref(),
                            2000
                        );
                        timeout_closure.forget();
                    }) as Box<dyn FnMut(JsValue)>)
                };

                let error_callback = {
                    let copy_message_clone = copy_message.clone();
                    Closure::wrap(Box::new(move |_: JsValue| {
                        copy_message_clone.set("Failed to copy to clipboard".to_string());
                    }) as Box<dyn FnMut(JsValue)>)
                };

                let promise = clipboard.write_text(&svg_content);
                let _ = promise.then(&success_callback).catch(&error_callback);
                
                // Prevent the closures from being garbage collected
                success_callback.forget();
                error_callback.forget();
            }
        })
    };

    let handle_style_change = {
        let stroke_style = stroke_style.clone();
        let update_svg_paths = update_svg_paths.clone();
        Callback::from(move |e: Event| {
            let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
            let value = select.value();
            
            let new_style = match value.as_str() {
                "default" => StrokeStyle::Default,
                "default-stroke" => StrokeStyle::DefaultWithStroke,
                "tapered" => StrokeStyle::Tapered,
                "tapered-stroke" => StrokeStyle::TaperedWithStroke,
                "thin" => StrokeStyle::Thin,
                "thin-stroke" => StrokeStyle::ThinWithStroke,
                "thick" => StrokeStyle::Thick,
                "thick-stroke" => StrokeStyle::ThickWithStroke,
                _ => StrokeStyle::Default,
            };
            
            stroke_style.set(new_style);
            // Immediately update all paths with the new style
            update_svg_paths();
        })
    };

    html! {
        <div class="interactive-drawing-section">
            <div style="display: flex; align-items: center; gap: 10px; margin-bottom: 10px;">
                <h2 style="margin: 0;">{ "Try drawing below" }</h2>
                if !copy_message.is_empty() {
                    <div class="format-box copied" style="
                        padding: 4px 12px;
                        background-color: #223322;
                        border: 1px solid #4caf50;
                        border-radius: 4px;
                        color: #4caf50;
                        font-size: 14px;
                        height: fit-content;
                    ">
                        { &*copy_message }
                    </div>
                }
            </div>
            <div style="position: relative; height: 400px; width: 100%; margin: 0 auto; background-color: #1E1E1E; border: 2px solid rgb(51, 51, 51);">
                <canvas ref={canvas_ref}
                    width="1000"
                    height="400"
                    style="position: absolute; z-index: 20; top: 0; left: 0; width: 100%; height: 400px; display: block;"
                    {onmousedown}
                    {onmousemove}
                    onmouseup={onmouseup.clone()}
                    onmouseleave={onmouseup}
                />
                <svg 
                    width="1000" 
                    height="400" 
                    style="position: absolute; z-index: 10; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none;"
                    viewBox="0 0 1000 400"
                    preserveAspectRatio="none"
                >
                    {
                        svg_paths.iter().map(|(path, is_stroke_only)| {
                            html! {
                                <path 
                                    d={path.clone()} 
                                    fill={if *is_stroke_only { "none" } else { "white" }} 
                                    stroke="WHITE" 
                                    stroke-width={if *is_stroke_only { "2.5" } else { "1" }}
                                    stroke-linejoin="round"
                                    stroke-linecap="round"
                                />
                            }
                        }).collect::<Html>()
                    }
                </svg>
            </div>
            <div style="display: flex; justify-content: center; align-items: center; gap: 10px; margin-top: 10px;">
                <label style="display: flex; align-items: center; gap: 5px;">
                    <input 
                        type="checkbox" 
                        checked={*show_trace}
                        onchange={toggle_trace}
                    />
                    { "Show drawing trace" }
                </label>
                <select 
                    onchange={handle_style_change}
                    style="padding: 8px 16px; border-radius: 4px; background-color: #1E1E1E; color: white; border: 1px solid #444; appearance: none; -webkit-appearance: none; background-image: url('data:image/svg+xml;utf8,<svg fill=\"white\" height=\"24\" viewBox=\"0 0 24 24\" width=\"24\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M7 10l5 5 5-5z\"/></svg>'); background-repeat: no-repeat; background-position: right 8px center; padding-right: 32px;"
                >
                    <option value="default" selected={*stroke_style == StrokeStyle::Default}>{ "Default" }</option>
                    <option value="default-stroke" selected={*stroke_style == StrokeStyle::DefaultWithStroke}>{ "Default with Stroke" }</option>
                    <option value="tapered" selected={*stroke_style == StrokeStyle::Tapered}>{ "Tapered" }</option>
                    <option value="tapered-stroke" selected={*stroke_style == StrokeStyle::TaperedWithStroke}>{ "Tapered with Stroke" }</option>
                    <option value="thin" selected={*stroke_style == StrokeStyle::Thin}>{ "Thin" }</option>
                    <option value="thin-stroke" selected={*stroke_style == StrokeStyle::ThinWithStroke}>{ "Thin with Stroke" }</option>
                    <option value="thick" selected={*stroke_style == StrokeStyle::Thick}>{ "Thick" }</option>
                    <option value="thick-stroke" selected={*stroke_style == StrokeStyle::ThickWithStroke}>{ "Thick with Stroke" }</option>
                </select>
                <button onclick={clear_drawing} style="padding: 8px 16px;">{ "Clear Drawing" }</button>
                <button onclick={copy_svg} style="padding: 8px 16px;">{ "Copy SVG" }</button>
            </div>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
            <div class="header">
                <h1>{ "Perfect Freehand SVG Examples" }</h1>
                <p>{ "Comparison of different SVG rendering options from the Rust port of perfect-freehand" }</p>
            </div>
            
            <div class="nav">
                <ul>
                    <li><a href="#sample">{ "Sample Drawing" }</a></li>
                    <li><a href="#inputs">{ "Basic Examples" }</a></li>
                    <li><a href="#flash">{ "Flash Drawing" }</a></li>
                    <li><a href="#corners">{ "Corners Drawing" }</a></li>
                    <li><a href="#edge-cases">{ "Edge Cases" }</a></li>
                </ul>
            </div>

            // Inserting the interactive DrawingBoard above the sample drawing section
            <DrawingBoard />

            /* Sample Drawing Section - Moved to the top */
            <div id="sample" class="section">
                <h2>{ "Sample Drawing (sample.json)" }</h2>
                <div class="grid">
                    <SvgExample 
                        title="Default" 
                        src="examples/svg/sample_default.svg" 
                        filename="sample_default.svg"
                    />
                    <SvgExample 
                        title="Default with Stroke" 
                        src="examples/svg/sample_default_stroke.svg" 
                        filename="sample_default_stroke.svg"
                    />
                    <SvgExample 
                        title="Tapered" 
                        src="examples/svg/sample_default_tapered.svg" 
                        filename="sample_default_tapered.svg"
                    />
                    <SvgExample 
                        title="Tapered with Stroke" 
                        src="examples/svg/sample_default_tapered_stroke.svg" 
                        filename="sample_default_tapered_stroke.svg"
                    />
                    <SvgExample 
                        title="Thin" 
                        src="examples/svg/sample_thin.svg" 
                        filename="sample_thin.svg"
                    />
                    <SvgExample 
                        title="Thin with Stroke" 
                        src="examples/svg/sample_thin_stroke.svg" 
                        filename="sample_thin_stroke.svg"
                    />
                    <SvgExample 
                        title="Thick" 
                        src="examples/svg/sample_thick.svg" 
                        filename="sample_thick.svg"
                    />
                    <SvgExample 
                        title="Thick with Stroke" 
                        src="examples/svg/sample_thick_stroke.svg" 
                        filename="sample_thick_stroke.svg"
                    />
                </div>
            </div>

            <div id="inputs" class="section">
                <h2>{ "Basic Examples (inputs.json)" }</h2>
                <div class="section">
                    <h2>{ "Many Points" }</h2>
                    <div class="grid">
                        <SvgExample 
                            title="Standard - Fill" 
                            src="examples/svg/manyPoints.svg" 
                            filename="manyPoints.svg"
                        />
                        <SvgExample 
                            title="Standard - Stroke" 
                            src="examples/svg/manyPoints_with_stroke.svg" 
                            filename="manyPoints_with_stroke.svg"
                        />
                        <SvgExample 
                            title="Tapered - Fill" 
                            src="examples/svg/manyPoints_tapered.svg" 
                            filename="manyPoints_tapered.svg"
                        />
                        <SvgExample 
                            title="Tapered - Stroke" 
                            src="examples/svg/manyPoints_tapered_with_stroke.svg" 
                            filename="manyPoints_tapered_with_stroke.svg"
                        />
                    </div>
                </div>

                <div class="section">
                    <h2>{ "Number Pairs" }</h2>
                    <div class="grid">
                        <SvgExample 
                            title="Standard - Fill" 
                            src="examples/svg/numberPairs.svg" 
                            filename="numberPairs.svg"
                        />
                        <SvgExample 
                            title="Standard - Stroke" 
                            src="examples/svg/numberPairs_with_stroke.svg" 
                            filename="numberPairs_with_stroke.svg"
                        />
                        <SvgExample 
                            title="Tapered - Fill" 
                            src="examples/svg/numberPairs_tapered.svg" 
                            filename="numberPairs_tapered.svg"
                        />
                        <SvgExample 
                            title="Tapered - Stroke" 
                            src="examples/svg/numberPairs_tapered_with_stroke.svg" 
                            filename="numberPairs_tapered_with_stroke.svg"
                        />
                    </div>
                </div>

                <div class="section">
                    <h2>{ "Object Pairs" }</h2>
                    <div class="grid">
                        <SvgExample 
                            title="Standard - Fill" 
                            src="examples/svg/objectPairs.svg" 
                            filename="objectPairs.svg"
                        />
                        <SvgExample 
                            title="Standard - Stroke" 
                            src="examples/svg/objectPairs_with_stroke.svg" 
                            filename="objectPairs_with_stroke.svg"
                        />
                        <SvgExample 
                            title="Tapered - Fill" 
                            src="examples/svg/objectPairs_tapered.svg" 
                            filename="objectPairs_tapered.svg"
                        />
                        <SvgExample 
                            title="Tapered - Stroke" 
                            src="examples/svg/objectPairs_tapered_with_stroke.svg" 
                            filename="objectPairs_tapered_with_stroke.svg"
                        />
                    </div>
                </div>

                <div class="section">
                    <h2>{ "With Duplicates" }</h2>
                    <div class="grid">
                        <SvgExample 
                            title="Standard - Fill" 
                            src="examples/svg/withDuplicates.svg" 
                            filename="withDuplicates.svg"
                        />
                        <SvgExample 
                            title="Standard - Stroke" 
                            src="examples/svg/withDuplicates_with_stroke.svg" 
                            filename="withDuplicates_with_stroke.svg"
                        />
                        <SvgExample 
                            title="Tapered - Fill" 
                            src="examples/svg/withDuplicates_tapered.svg" 
                            filename="withDuplicates_tapered.svg"
                        />
                        <SvgExample 
                            title="Tapered - Stroke" 
                            src="examples/svg/withDuplicates_tapered_with_stroke.svg" 
                            filename="withDuplicates_tapered_with_stroke.svg"
                        />
                    </div>
                </div>
            </div>

            /* Flash Drawing Section */
            <div id="flash" class="section">
                <h2>{ "Flash Drawing (flash.json)" }</h2>
                <div class="grid">
                    <SvgExample 
                        title="Default" 
                        src="examples/svg/flash_default.svg" 
                        filename="flash_default.svg"
                    />
                    <SvgExample 
                        title="Default with Stroke" 
                        src="examples/svg/flash_default_stroke.svg" 
                        filename="flash_default_stroke.svg"
                    />
                    <SvgExample 
                        title="Tapered" 
                        src="examples/svg/flash_default_tapered.svg" 
                        filename="flash_default_tapered.svg"
                    />
                    <SvgExample 
                        title="Tapered with Stroke" 
                        src="examples/svg/flash_default_tapered_stroke.svg" 
                        filename="flash_default_tapered_stroke.svg"
                    />
                    <SvgExample 
                        title="Simulated Pressure" 
                        src="examples/svg/flash_simulated.svg" 
                        filename="flash_simulated.svg"
                    />
                    <SvgExample 
                        title="Simulated Pressure with Stroke" 
                        src="examples/svg/flash_simulated_stroke.svg" 
                        filename="flash_simulated_stroke.svg"
                    />
                </div>
            </div>

            /* Corners Drawing Section */
            <div id="corners" class="section">
                <h2>{ "Corners Drawing (corners.json)" }</h2>
                <div class="grid">
                    <SvgExample 
                        title="Default" 
                        src="examples/svg/corners_default.svg" 
                        filename="corners_default.svg"
                    />
                    <SvgExample 
                        title="Default with Stroke" 
                        src="examples/svg/corners_default_stroke.svg" 
                        filename="corners_default_stroke.svg"
                    />
                    <SvgExample 
                        title="Tapered" 
                        src="examples/svg/corners_default_tapered.svg" 
                        filename="corners_default_tapered.svg"
                    />
                    <SvgExample 
                        title="Tapered with Stroke" 
                        src="examples/svg/corners_default_tapered_stroke.svg" 
                        filename="corners_default_tapered_stroke.svg"
                    />
                    <SvgExample 
                        title="Thick" 
                        src="examples/svg/corners_thick.svg" 
                        filename="corners_thick.svg"
                    />
                    <SvgExample 
                        title="Thick with Stroke" 
                        src="examples/svg/corners_thick_stroke.svg" 
                        filename="corners_thick_stroke.svg"
                    />
                </div>
            </div>

            <div id="edge-cases" class="section">
                <h2>{ "Edge Cases" }</h2>
                <div class="grid">
                    <SvgExample 
                        title="One Point" 
                        src="examples/svg/onePoint.svg" 
                        filename="onePoint.svg"
                    />
                    <SvgExample 
                        title="One Point - Stroke" 
                        src="examples/svg/onePoint_with_stroke.svg" 
                        filename="onePoint_with_stroke.svg"
                    />
                    <SvgExample 
                        title="Two Points" 
                        src="examples/svg/twoPoints.svg" 
                        filename="twoPoints.svg"
                    />
                    <SvgExample 
                        title="Two Points - Stroke" 
                        src="examples/svg/twoPoints_with_stroke.svg" 
                        filename="twoPoints_with_stroke.svg"
                    />
                    <SvgExample 
                        title="Two Equal Points" 
                        src="examples/svg/twoEqualPoints.svg" 
                        filename="twoEqualPoints.svg"
                    />
                    <SvgExample 
                        title="Two Equal Points - Stroke" 
                        src="examples/svg/twoEqualPoints_with_stroke.svg" 
                        filename="twoEqualPoints_with_stroke.svg"
                    />
                </div>
            </div>
        </div>
    }
}