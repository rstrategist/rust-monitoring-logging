# Rust Monitoring and Logging

## Example project: Redact Personal Identifiable information as a service

This example project will allow you to explore how to add and enable different logging and monitoring techniques to a Rust project. The simple microservice accepts a JSON string as input to the `/redact` endpoint and return a JSON string with the PII redacted based on regular expressions stored as JSON.

This project is part of a course on Rust For DevOps focused on principles like logging, testing, monitoring, automation, CI/CD, containers, and deployment.

## Contents

There are several examples located in the [./examples](./examples) directory.

### Prometheus and Grafana

- [Introduction to Prometheus](./examples/1-prometheus/)
- [Integrating Prometheus with Grafana](./examples/2-prometheus-grafana/)
- [Creating custom endpoints](./examples/3-custom-endpoint/)

### Adding Logging

- [Adding logging to your Rust application](./examples/4-adding-logging/)
- [Controlling verbosity in Rust](./examples/5-controlling-verbosity/)

### Structured Logging

- [Exploring structured logging in Rust](./examples/6-structured-logging/)

## Resources

Explore additional content that you can use to learn more about the topics covered.

### Coursera Courses

- Rust for DevOps Course
  -- [week 1](https://github.com/alfredodeza/rust-setup)
  -- [week 2](https://github.com/alfredodeza/rust-monitoring-logging)
  -- [week 3](https://github.com/alfredodeza/rust-systems-programming/)
  -- [week 4](https://github.com/alfredodeza/advanced-ci-cd-concepts)

- [Linux and Bash for Data Engineering](https://www.coursera.org/learn/linux-and-bash-for-data-engineering-duke)
- [Open Source Platforms for MLOps](https://www.coursera.org/learn/open-source-platforms-duke)
- [Python Essentials for MLOps](https://www.coursera.org/learn/python-essentials-mlops-duke)
- [Web Applications and Command-Line tools for Data Engineering](https://www.coursera.org/learn/web-app-command-line-tools-for-data-engineering-duke)
- [Python and Pandas for Data Engineering](https://www.coursera.org/learn/python-and-pandas-for-data-engineering-duke)
- [Scripting with Python and SQL for Data Engineering](https://www.coursera.org/learn/scripting-with-python-sql-for-data-engineering-duke)
