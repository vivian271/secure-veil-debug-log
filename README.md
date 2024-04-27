# Secure and Private Data Handling in Software Development

**Authors:** Riyaz Rafi Ahmed, Wenhan Zhou, Maitreya Sawai, Elliot Chen

## Introduction

The project aims to enhance data privacy in software development by implementing advanced constructs for differential privacy in Rust programming libraries. Specifically, we focus on incorporating techniques to protect sensitive data in debug logs. This is crucial as debug logs often contain personal and confidential information that should not be exposed or stored insecurely. By applying differential privacy techniques within Rust's Veil framework, developers can securely handle various types of sensitive data throughout the software development lifecycle without sacrificing usability or performance.


## Background

### Tools and Libraries Used:

- **Veil:** Rust programming library for secure data redaction.
- **Differential Privacy Techniques:** Privacy-preserving techniques that add noise to query results or data aggregates while preserving statistical properties.
- **Rust Programming Language:** Used for implementing differential privacy and secure data handling.

### Dataset Used in the Project:

**Dataset:**   [Cardiovascular Disease Dataset](https://www.kaggle.com/datasets/sulianova/cardiovascular-disease-dataset)

**Source:** Kaggle  
**Description:** Contains data on health indicators of various individuals related to cardiovascular diseases.
**Sensitive Parameters:** Age, Height, and Weight are considered sensitive parameters in our context.

| Attributes | Description |
| --- | --- |
| id | Unique identifier for each participant.|
| age | Age of the participant in days. |
| gender| Gender of the participant (1: female, 2: male).|
| height| Height of the participant in centimeters |
| weight | Weight of the participant in kilograms |
| ap_hi| Systolic blood pressure |
| ap_lo| Diastolic blood pressure |
| cholesterol| Cholesterol level (1: normal, 2: above normal, 3: well above normal)|
| gluc| Glucose level (1: normal, 2: above normal, 3: well above normal)|
| smoke| Smoking status (0: non-smoker, 1: smoker) |
|alco| Alcohol intake (0: non-drinker, 1: drinker) |
| active| Physical activity (0: inactive, 1: active)|
| cardio| Presence or absence of cardiovascular disease (0: no, 1: yes) |
| git diff | Show file differences that haven't been staged |




## Overview

### Initial Planning and Tasks:

1. Implement advanced constructs for differential privacy.
2. Incorporate techniques to protect sensitive data in debug logs.
3. Leverage Rust's Veil library for efficient redaction of sensitive data.
4. Ensure the implementation is configurable and efficient.

### Implementation Overview:

- Utilized Rust's Veil library for redaction of sensitive data in debug logs.
- Applied differential privacy techniques to age, height, and weight fields in the dataset.
- Modified Veil's source code to integrate differential privacy functionality.

### Workflow Diagram:
![alt text](https://github.com/vivian271/secure-veil-debug-log/blob/secure-debug-log/Workflow%20Diagram.png)

## Implementation 1: DP Implementation w/ dataset using Rust's Veil [[secure-veil-debug-log branch](https://github.com/vivian271/secure-veil-debug-log)]
#### Implementation Details

This code implements a secure and privacy-aware data handling mechanism in Rust for managing patient information. It begins by defining a `Patient` struct with various fields representing patient attributes such as age, gender, height, weight, and health indicators. The `Redact` trait from the `veil` crate is derived for the `Patient` struct to specify redaction rules for sensitive fields, ensuring that certain information like age and gender are obscured in debug logs. Additionally, the `add_dp_noise` method is implemented within the `Patient` struct to add Laplacian noise for differential privacy to the age, height, and weight fields. This noise helps protect the privacy of individual records while preserving the overall statistical properties of the data.

In the `main` function, the code reads patient data from a CSV file, parses it into the `Patient` struct fields, and then applies differential privacy techniques to the age, height, and weight attributes. Before and after adding noise, the code prints the values of these attributes to demonstrate the impact of the privacy-preserving mechanism. Finally, it logs the patient details using the `debug!` macro from the `log` crate, providing a way to track and analyze the application's behavior while ensuring sensitive information remains protected.

### How to Run

1. Switch to `secure-veil-debug-log` branch
2. Make sure rust is installed in your system.
3. Open the "foo" folder in VS-Code
4. run `cargo build` from the "foo" folder
5. run `cargo run`

## Implementation 2: Rust's Veil Library Implementation [[master branch](https://github.com/vivian271/secure-veil-debug-log/tree/master)]
#### Implementation Details
This implementation takes the existing Rust's Veil code and improves the current redact functionality by adding differential privacy for 'Debug' in rust. The challenge we faced was that many files in the code depend on each other, which can lead to subtle errors between them.

In the `src` section, changes were made to 'redactable.rs'. This file handles types of data that are sensitive or contain personally identifiable information (PII) that need to be concealed. There's a simple way to format these types to hide sensitive details. Additionally, a method was introduced to slightly modify the data when it's sensitive, such as altering someone's age. To add privacy protection, a random noise generator was developed to append a small random number (ranging from 0 to 5) to the data. Furthermore, we included Laplace noise option since it can apply to both numerical values and strings.

Within `veil-macros`, adjustments were made to 'Enum.rs'. This segment focuses on concealing enum variants and  establishes a method to display these enum variants and its details while keeping the sensitive aspects hidden. In order to add DP, a tool called LaplaceNoiser was imported from `diff_priv` library, and an instance named 'noiser' was created. In particular, 'variant_bodies' were modified to introduce this special form of noise to the sensitive portions of the data. In this instance, it was assumed that the sensitive part is referred to as 'numbers', which could be replaced with another sensitive part name in the dataset.

Through this implementation, it becomes simpler to conceal and manage sensitive information in Rust code, enhancing its safety and reliability.

### How to Run

1. Switch to `master` branch
2. ```cargo run --package veil --example customer```
3. The updated veil can be run on the example customer.rs successfully
