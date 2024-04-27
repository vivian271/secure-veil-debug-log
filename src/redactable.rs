use rand::Rng;

pub trait Redactable {
    /// Returns this value formatted as a string with all PII/sensitive data redacted.
    fn redact(&self) -> String {
        let mut buffer = String::new();

        self.redact_into(&mut buffer)
            .expect("writing to a String should never fail");

        // Add Laplace noise
        add_laplace_noise(&mut buffer, 3.0, 0.45);
        // add_random_noise(&mut buffer);

        buffer
    }

    /// Writes this value formatted as a string with all PII/sensitive data redacted into the given buffer.
    fn redact_into(&self, buffer: &mut dyn std::fmt::Write) -> std::fmt::Result;
}

// Function to add noise to the redacted string
fn add_random_noise(buffer: &mut String) {
    // Generate random noise, here we simply add a random number to each character
    let mut rng = rand::thread_rng();
    let mut new_buffer = String::new();
    for c in buffer.chars() {
        let noise: u8 = rng.gen_range(0..=5); // Adjust this range based on your requirement
        let new_c = char::from_u32(c as u32 + noise as u32).unwrap_or(c);
        new_buffer.push(new_c);
    }
    *buffer = new_buffer;
}

// Function to add Laplace noise to the redacted string
fn add_laplace_noise(buffer: &mut String, sensitivity: f64, epsilon: f64) {
    // Convert the string to bytes
    let mut bytes = buffer.as_bytes().to_vec();

    // Generate Laplace noise and add it to the buffer
    for byte in bytes.iter_mut() {
        let laplace_noise = generate_laplace_noise(sensitivity, epsilon);
        *byte = (*byte as i32 + laplace_noise) as u8;
    }

    // Convert bytes back to string
    *buffer = String::from_utf8_lossy(&bytes).to_string();
}

// Function to generate Laplace noise
fn generate_laplace_noise(sensitivity: f64, epsilon: f64) -> i32 {
    // Calculate the scale parameter for the Laplace distribution
    let scale = sensitivity / epsilon;

    // Generate Laplace-distributed noise
    let uniform_sample: f64 = rand::random();
    let noise = if uniform_sample < 0.5 {
        (-scale * uniform_sample.ln()).round() as i32
    } else {
        (scale * uniform_sample.ln()).round() as i32
    };

    noise
}
