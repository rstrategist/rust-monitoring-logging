/*
Project: Rust Monitoring and Logging Microservice

Description: This microservice is built with Rust and Actix Web.
It provides endpoints for redacting personal identifiable information (PII) from text,
as well as monitoring the service with Prometheus, and checking the health of the service.
*/

// Import necessary crates and modules
use actix_web::{web, App, HttpResponse, HttpServer, Responder}; // Actix Web framework for building web applications
use actix_web_prom::PrometheusMetricsBuilder; // Library for Prometheus metrics integration
use psutil::memory; // Library for system information like memory usage
use redactr::load_rule_configs; // Custom module for loading redaction rules
use regex::Regex; // Library for regular expressions
use serde::Serialize; // Library for serialising data structures
use std::time::{SystemTime, UNIX_EPOCH}; // Standard library for time handling

// Define the structure for health check JSON response endpoint
#[derive(Serialize)]
struct HealthCheck {
    name: String,   // Name of the health check
    status: String, // Status of the health check
}

// Main function to start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize Prometheus metrics
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone()) // Add Prometheus middleware
            .route("/health", web::get().to(health_check)) // Health check endpoint
            .route("/redact", web::post().to(redact)) // Redaction endpoint
    })
    .bind("127.0.0.1:8080")? // Bind to address and port
    .run()
    .await
}

// Health check handler
async fn health_check() -> impl Responder {
    let response = HealthCheck {
        name: "Rust Monitoring and Logging Microservice".to_string(),
        status: "Healthy".to_string(),
    };
    HttpResponse::Ok().json(response) // Return JSON response
}

// Redaction handler
async fn redact(text: web::Json<String>) -> impl Responder {
    let rules = load_rule_configs(); // Load redaction rules
    let mut redactor = Redactor::new(rules); // Initialise redactor
    let redacted_text = redactor.redact(&text); // Redact text
    HttpResponse::Ok().body(redacted_text) // Return redacted text
}
