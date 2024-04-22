/// Types that are sensitive data or PII (Personally Identifiable Information) and can be redact-formatted.
//
/// This trait can be manually implemented or derived using the [`Redactable`](derive.Redactable.html) macro.
// pub trait Redactable {
//     /// Returns this value formatted as a string with all PII/sensitive data redacted.
//     fn redact(&self) -> String {
//         let mut buffer = String::new();

//         self.redact_into(&mut buffer)
//             .expect("writing to a String should never fail");

//         buffer
//     }

//     /// Writes this value formatted as a string with all PII/sensitive data redacted into the given buffer.
//     fn redact_into(&self, buffer: &mut dyn std::fmt::Write) -> std::fmt::Result;
// }
use rand::Rng;

// Import rand crate for generating random numbers

pub trait Redactable {
    /// Returns this value formatted as a string with all PII/sensitive data redacted.
    fn redact(&self) -> String {
        let mut buffer = String::new();

        self.redact_into(&mut buffer)
            .expect("writing to a String should never fail");

        // Add differential privacy noise
        add_noise(&mut buffer);

        buffer
    }

    /// Writes this value formatted as a string with all PII/sensitive data redacted into the given buffer.
    fn redact_into(&self, buffer: &mut dyn std::fmt::Write) -> std::fmt::Result;
}

// Function to add noise to the redacted string
fn add_noise(buffer: &mut String) {
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


