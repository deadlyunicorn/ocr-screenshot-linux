use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, Button, Frame, Image as GtkImage,
    Label, Orientation, ScrolledWindow, TextView, TextBuffer, PolicyType, gdk, Align,
};
use gdk::Display;
use image::{DynamicImage, GrayImage, imageops::FilterType};
use imageproc::contrast::{threshold, ThresholdType};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

const APP_ID: &str = "com.github.ocr_screenshot";

fn main() {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    // Create main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("OCR Screenshot Tool")
        .default_width(900)
        .default_height(700)
        .build();

    // Main vertical box
    let main_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    // Header
    let header_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .build();

    let title_label = Label::builder()
        .label("<span size='x-large' weight='bold'>OCR Screenshot Tool</span>")
        .use_markup(true)
        .halign(Align::Start)
        .hexpand(true)
        .build();

    let instruction_label = Label::builder()
        .label("Press Ctrl+V to paste an image from clipboard")
        .halign(Align::End)
        .build();
    instruction_label.add_css_class("dim-label");

    header_box.append(&title_label);
    header_box.append(&instruction_label);
    main_box.append(&header_box);

    // Image display area
    let image_frame = Frame::builder()
        .label("Image")
        .vexpand(true)
        .build();

    let image_scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Automatic)
        .vscrollbar_policy(PolicyType::Automatic)
        .min_content_height(250)
        .build();

    let image_widget = GtkImage::builder()
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    let placeholder_label = Label::builder()
        .label("<span size='large' foreground='gray'>No image loaded\\nPress Ctrl+V to paste from clipboard</span>")
        .use_markup(true)
        .justify(gtk4::Justification::Center)
        .build();

    // Stack to switch between placeholder and image
    let image_stack = gtk4::Stack::new();
    image_stack.add_named(&placeholder_label, Some("placeholder"));
    image_stack.add_named(&image_widget, Some("image"));
    image_stack.set_visible_child_name("placeholder");

    image_scrolled.set_child(Some(&image_stack));
    image_frame.set_child(Some(&image_scrolled));
    main_box.append(&image_frame);

    // Button box
    let button_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(5)
        .halign(Align::Center)
        .build();

    let ocr_button = Button::builder()
        .label("Perform OCR")
        .sensitive(false)
        .build();

    let copy_button = Button::builder()
        .label("Copy Text")
        .sensitive(false)
        .build();

    let clear_button = Button::builder()
        .label("Clear")
        .sensitive(false)
        .build();

    button_box.append(&ocr_button);
    button_box.append(&copy_button);
    button_box.append(&clear_button);
    main_box.append(&button_box);

    // Text display area
    let text_frame = Frame::builder()
        .label("Extracted Text")
        .vexpand(true)
        .build();

    let text_scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Automatic)
        .vscrollbar_policy(PolicyType::Automatic)
        .min_content_height(200)
        .build();

    let text_view = TextView::builder()
        .wrap_mode(gtk4::WrapMode::Word)
        .editable(false)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();

    let text_buffer = text_view.buffer();
    text_buffer.set_text("OCR results will appear here...");

    text_scrolled.set_child(Some(&text_view));
    text_frame.set_child(Some(&text_scrolled));
    main_box.append(&text_frame);

    // Status bar
    let status_label = Label::builder()
        .label("Ready. Press Ctrl+V to paste an image.")
        .halign(Align::Start)
        .build();
    main_box.append(&status_label);

    window.set_child(Some(&main_box));

    // Shared state
    let current_image: Arc<Mutex<Option<DynamicImage>>> = Arc::new(Mutex::new(None));

    // Setup keyboard shortcuts
    let key_controller = gtk4::EventControllerKey::new();
    
    let image_clone = current_image.clone();
    let ocr_btn_clone = ocr_button.clone();
    let clear_btn_clone = clear_button.clone();
    let image_widget_clone = image_widget.clone();
    let image_stack_clone = image_stack.clone();
    let status_clone = status_label.clone();
    let text_buffer_clone = text_buffer.clone();
    let copy_btn_clone = copy_button.clone();

    key_controller.connect_key_pressed(move |_, key, _code, modifier| {
        // Check for Ctrl+V
        if modifier.contains(gdk::ModifierType::CONTROL_MASK) && 
           (key == gdk::Key::v || key == gdk::Key::V) {
            
            status_clone.set_label("Reading clipboard...");
            
            // Get clipboard
            let clipboard = gdk::Display::default()
                .and_then(|d| Some(d.clipboard()))
                .expect("Failed to get clipboard");
            
            let image_clone2 = image_clone.clone();
            let ocr_btn_clone2 = ocr_btn_clone.clone();
            let clear_btn_clone2 = clear_btn_clone.clone();
            let image_widget_clone2 = image_widget_clone.clone();
            let image_stack_clone2 = image_stack_clone.clone();
            let status_clone2 = status_clone.clone();
            let text_buffer_clone2 = text_buffer_clone.clone();
            let copy_btn_clone2 = copy_btn_clone.clone();

            clipboard.read_texture_async(None::<&gtk4::gio::Cancellable>, move |result| {
                match result {
                    Ok(Some(texture)) => {
                        // Convert texture to image
                        if let Some(img) = texture_to_image(&texture) {
                            let (width, height) = (img.width(), img.height());
                            
                            // Store image
                            *image_clone2.lock().unwrap() = Some(img.clone());
                            
                            // Display image
                            let display_texture = image_to_texture(&img);
                            image_widget_clone2.set_paintable(Some(&display_texture));
                            image_stack_clone2.set_visible_child_name("image");
                            
                            // Enable buttons
                            ocr_btn_clone2.set_sensitive(true);
                            clear_btn_clone2.set_sensitive(true);
                            
                            status_clone2.set_label(&format!("Image loaded: {}x{} pixels", width, height));
                            
                            // Auto-perform OCR
                            perform_ocr_async(
                                img,
                                text_buffer_clone2,
                                copy_btn_clone2,
                                status_clone2.clone(),
                                ocr_btn_clone2.clone()
                            );
                        } else {
                            status_clone2.set_label("Failed to convert clipboard image");
                        }
                    }
                    Ok(None) => {
                        status_clone2.set_label("No image found in clipboard");
                    }
                    Err(_) => {
                        status_clone2.set_label("Error reading from clipboard");
                    }
                }
            });
            
            return glib::Propagation::Stop;
        }
        
        glib::Propagation::Proceed
    });

    window.add_controller(key_controller);

    // OCR button handler
    let image_clone2 = current_image.clone();
    let text_buffer_clone2 = text_buffer.clone();
    let copy_btn_clone2 = copy_button.clone();
    let status_clone2 = status_label.clone();
    let ocr_btn_clone2 = ocr_button.clone();

    ocr_button.connect_clicked(move |_| {
        if let Some(img) = image_clone2.lock().unwrap().clone() {
            perform_ocr_async(
                img,
                text_buffer_clone2.clone(),
                copy_btn_clone2.clone(),
                status_clone2.clone(),
                ocr_btn_clone2.clone()
            );
        }
    });

    // Copy button handler
    let text_buffer_clone3 = text_buffer.clone();
    let status_clone3 = status_label.clone();
    
    copy_button.connect_clicked(move |_| {
        let text = text_buffer_clone3.text(
            &text_buffer_clone3.start_iter(),
            &text_buffer_clone3.end_iter(),
            false
        );
        
        if let Some(display) = Display::default() {
            display.clipboard().set_text(&text);
            status_clone3.set_label("Text copied to clipboard");
        }
    });

    // Clear button handler
    let image_clone3 = current_image.clone();
    let image_stack_clone2 = image_stack.clone();
    let text_buffer_clone4 = text_buffer.clone();
    let ocr_btn_clone3 = ocr_button.clone();
    let copy_btn_clone3 = copy_button.clone();
    let clear_btn_clone2 = clear_button.clone();
    let status_clone4 = status_label.clone();

    clear_button.connect_clicked(move |_| {
        *image_clone3.lock().unwrap() = None;
        image_stack_clone2.set_visible_child_name("placeholder");
        text_buffer_clone4.set_text("OCR results will appear here...");
        ocr_btn_clone3.set_sensitive(false);
        copy_btn_clone3.set_sensitive(false);
        clear_btn_clone2.set_sensitive(false);
        status_clone4.set_label("Ready. Press Ctrl+V to paste an image.");
    });

    window.present();
}

fn texture_to_image(texture: &gdk::Texture) -> Option<DynamicImage> {
    
    // Save texture to memory buffer
    let width = texture.width() as u32;
    let height = texture.height() as u32;
    let stride = (width * 4) as usize; // RGBA
    let mut bytes = vec![0u8; stride * height as usize];
    
    texture.download(&mut bytes, stride);
    
    // Convert RGBA to image
    image::RgbaImage::from_raw(width, height, bytes)
        .map(DynamicImage::ImageRgba8)
}

fn image_to_texture(img: &DynamicImage) -> gdk::Texture {
    // Scale down if too large for display
    let display_img = if img.width() > 800 || img.height() > 400 {
        let scale = f32::min(800.0 / img.width() as f32, 400.0 / img.height() as f32);
        let new_width = (img.width() as f32 * scale) as u32;
        let new_height = (img.height() as f32 * scale) as u32;
        img.resize(new_width, new_height, FilterType::Lanczos3)
    } else {
        img.clone()
    };
    
    let rgba = display_img.to_rgba8();
    let bytes = glib::Bytes::from(&rgba.as_raw()[..]);
    
    gdk::MemoryTexture::new(
        rgba.width() as i32,
        rgba.height() as i32,
        gdk::MemoryFormat::R8g8b8a8,
        &bytes,
        rgba.width() as usize * 4
    ).upcast()
}

fn preprocess_image(img: &DynamicImage) -> GrayImage {
    // Convert to grayscale
    let mut gray = img.to_luma8();
    
    // Apply simple thresholding (Otsu-like)
    let threshold_val = calculate_otsu_threshold(&gray);
    gray = threshold(&gray, threshold_val, ThresholdType::Binary);
    
    // Upscale if needed for better OCR
    let (width, height) = gray.dimensions();
    if width < 1500 || height < 1500 {
        let scale = f32::max(1500.0 / width as f32, f32::max(1500.0 / height as f32, 2.0));
        let new_width = (width as f32 * scale) as u32;
        let new_height = (height as f32 * scale) as u32;
        
        let resized = image::imageops::resize(
            &gray,
            new_width,
            new_height,
            FilterType::Lanczos3
        );
        return resized;
    }
    
    gray
}

fn calculate_otsu_threshold(img: &GrayImage) -> u8 {
    // Calculate histogram
    let mut histogram = [0u32; 256];
    for pixel in img.pixels() {
        histogram[pixel[0] as usize] += 1;
    }
    
    let total_pixels = img.width() * img.height();
    let mut sum = 0.0;
    for i in 0..256 {
        sum += (i as f64) * (histogram[i] as f64);
    }
    
    let mut sum_b = 0.0;
    let mut w_b = 0;
    let mut max_variance = 0.0;
    let mut threshold = 0u8;
    
    for i in 0..256 {
        w_b += histogram[i];
        if w_b == 0 {
            continue;
        }
        
        let w_f = total_pixels - w_b;
        if w_f == 0 {
            break;
        }
        
        sum_b += (i as f64) * (histogram[i] as f64);
        let m_b = sum_b / w_b as f64;
        let m_f = (sum - sum_b) / w_f as f64;
        
        let variance = (w_b as f64) * (w_f as f64) * (m_b - m_f) * (m_b - m_f);
        
        if variance > max_variance {
            max_variance = variance;
            threshold = i as u8;
        }
    }
    
    threshold
}

fn perform_ocr_async(
    img: DynamicImage,
    text_buffer: TextBuffer,
    copy_button: Button,
    status_label: Label,
    ocr_button: Button
) {
    status_label.set_label("Performing OCR...");
    ocr_button.set_sensitive(false);
    
    // Create channel for sending results back to main thread
    let (tx, rx) = mpsc::sync_channel::<String>(1);
    
    // Clone widgets for main thread callback
    let text_buffer_clone = text_buffer.clone();
    let copy_button_clone = copy_button.clone();
    let status_label_clone = status_label.clone();
    let ocr_button_clone = ocr_button.clone();
    
    // Spawn worker thread
    thread::spawn(move || {
        eprintln!("OCR thread started");
        
        // Preprocess
        let processed_img = preprocess_image(&img);
        eprintln!("Image preprocessed: {}x{}", processed_img.width(), processed_img.height());
        
        // Encode as PNG for Tesseract
        let mut png_bytes = Vec::new();
        match DynamicImage::ImageLuma8(processed_img).write_to(&mut std::io::Cursor::new(&mut png_bytes), image::ImageFormat::Png) {
            Ok(_) => {
                eprintln!("Image encoded as PNG, size: {} bytes", png_bytes.len());
            },
            Err(e) => {
                eprintln!("Failed to encode PNG: {:?}", e);
                return;
            }
        }
        
        // Perform OCR
        match tesseract::Tesseract::new(None, Some("eng")) {
            Ok(api) => {
                eprintln!("Tesseract initialized");
                match api.set_image_from_mem(&png_bytes) {
                    Ok(api) => {
                        eprintln!("Image set in Tesseract");
                        match api.set_variable("tessedit_pageseg_mode", "4") {
                            Ok(api) => {
                                match api.set_variable("tessedit_ocr_engine_mode", "1") {
                                    Ok(mut api) => {
                                        eprintln!("Variables set, performing OCR...");
                                        let text = api.get_text().unwrap_or_else(|e| {
                                            eprintln!("OCR error: {:?}", e);
                                            String::from("OCR failed")
                                        });
                                        eprintln!("OCR complete, text length: {}", text.len());
                                        
                                        // Post-process
                                        let text = text.replace('{', "(").replace('}', ")");
                                        
                                        // Send result back to main thread
                                        if let Err(e) = tx.send(text) {
                                            eprintln!("Failed to send result: {:?}", e);
                                        } else {
                                            eprintln!("Result sent successfully");
                                        }
                                    },
                                    Err(e) => eprintln!("Failed to set OCR engine mode: {:?}", e),
                                }
                            },
                            Err(e) => eprintln!("Failed to set page seg mode: {:?}", e),
                        }
                    },
                    Err(e) => eprintln!("Failed to set image: {:?}", e),
                }
            },
            Err(e) => eprintln!("Failed to initialize Tesseract: {:?}", e),
        }
    });
    
    // Set up receiver on main thread
    let _source_id = glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        match rx.try_recv() {
            Ok(text) => {
                eprintln!("Received OCR result in main thread");
                let char_count = text.len();
                text_buffer_clone.set_text(&text);
                copy_button_clone.set_sensitive(true);
                ocr_button_clone.set_sensitive(true);
                status_label_clone.set_label(&format!("OCR complete. Extracted {} characters.", char_count));
                glib::ControlFlow::Break
            },
            Err(mpsc::TryRecvError::Empty) => {
                // Still waiting
                glib::ControlFlow::Continue
            },
            Err(mpsc::TryRecvError::Disconnected) => {
                eprintln!("Channel disconnected!");
                status_label_clone.set_label("OCR failed - thread disconnected");
                ocr_button_clone.set_sensitive(true);
                glib::ControlFlow::Break
            }
        }
    });
}
