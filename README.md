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

In order to run Prometheus and Grafana using docker compose, I had to run the following in GitHub codespaces:

### Step 1: Uninstall old versions of Docker, if any. I SKIPPED THIS STEP

### since the Dockerfile and Devcontainer set up a fresh environment.

### This ensures there are no conflicts with existing installations.

```
for pkg in docker.io docker-doc docker-compose docker-compose-v2 podman-docker containerd runc; do sudo apt-get remove -y $pkg; done
```

### Step 2: Update the package database

```
sudo apt-get update
```

### Step 3: Install required packages for Docker installation

```
sudo apt-get install -y ca-certificates curl gnupg lsb-release
```

### Step 4: Add Docker's official GPG key

```
sudo mkdir -p /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/debian/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
```

### Step 5: Set up the Docker stable repository

```
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
```

### Step 6: Update the package database with Docker packages from the newly added repo

```
sudo apt-get update
```

### Step 7: Install Docker Engine, CLI, and Containerd

```
sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin
```

### Step 8: Add your user to the docker group to manage Docker as a non-root user

```
sudo groupadd docker
sudo usermod -aG docker $USER
```

### Apply the new group membership

```
newgrp docker
```

### Step 9: Verify Docker installation by checking the version

```
docker version
```

## Coursera Courses

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
