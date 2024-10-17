Gracias por la aclaración. Aquí tienes el README actualizado para el proyecto Alveus, basado en la tecnología que has compartido:

# 🌊 Alveus - Reservoir Data Dashboard

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Technology](https://img.shields.io/badge/Tech-Svelte%20|%20Bulma%20|%20Rust-blue)
![Cloud](https://img.shields.io/badge/Cloud-Oracle%20Cloud-orange)

## 📜 Table of Contents

- [🌊 Alveus - Reservoir Data Dashboard](#-alveus---reservoir-data-dashboard)
  - [📖 Table of Contents](#-table-of-contents)
  - [🚀 About the Project](#-about-the-project)
    - [🔍 Features](#-features)
    - [🛠️ Technologies Used](#️-technologies-used)
  - [🌐 Live Demo](#-live-demo)
  - [💾 Installation](#-installation)
    - [⚙️ Prerequisites](#️-prerequisites)
    - [📦 Setup](#-setup)
  - [🖥️ Usage](#️-usage)
    - [🔗 Accessing the Web App](#-accessing-the-web-app)
    - [📊 Exploring Reservoir Data](#-exploring-reservoir-data)
  - [🔧 Configuration](#-configuration)
    - [📁 Environment Variables](#-environment-variables)
    - [🗄️ Database Setup](#️-database-setup)
  - [🧑‍💻 Contributing](#-contributing)
    - [📋 Guidelines](#-guidelines)
  - [📜 License](#-license)
  - [🙏 Acknowledgements](#-acknowledgements)

## 🚀 About the Project

Welcome to **Alveus**, the **Reservoir Data Dashboard**! This web application provides comprehensive information about nearby reservoirs, including current water levels, historical data, averages, and interactive graphs. Leveraging the power of Oracle Cloud for our database and hosting infrastructure, we've built a robust and scalable solution to deliver real-time and historical reservoir data to users.

![Dashboard Screenshot](./assets/dashboard-screenshot.png)

### 🔍 Features

- **Real-Time Data:** Access up-to-date information on water levels of nearby reservoirs.
- **Historical Data:** View and analyze past water levels to identify trends.
- **Averages & Statistics:** Get average water levels and other relevant statistics.
- **Interactive Graphs:** Visualize data through dynamic and responsive charts.
- **Responsive Design:** Built with Bulma to ensure compatibility across devices.
- **High Performance:** Backend developed in Rust for speed and efficiency.
- **Modern Frontend:** Utilizes Svelte for a reactive and seamless user experience.
- **Secure & Scalable:** Hosted on Oracle Cloud for reliability and scalability.

### 🛠️ Technologies Used

- **Frontend:**
  - [Svelte](https://svelte.dev/) - A radical new approach to building user interfaces.
  - [Bulma](https://bulma.io/) - A modern CSS framework based on Flexbox.
  
- **Backend:**
  - [Rust](https://www.rust-lang.org/) - A fast and memory-efficient programming language.
  
- **Database & Hosting:**
  - [Oracle Cloud](https://www.oracle.com/cloud/) - Comprehensive cloud services for databases and hosting.

## 🌐 Live Demo

Check out the live version of **Alveus** [here](https://your-live-demo-link.com).

## 💾 Installation

### ⚙️ Prerequisites

Before you begin, ensure you have met the following requirements:

- **Node.js** (v14 or later) installed. [Download Node.js](https://nodejs.org/)
- **Rust** installed. [Install Rust](https://www.rust-lang.org/tools/install)
- An **Oracle Cloud** account with access to set up databases and instances.
- **Git** installed. [Download Git](https://git-scm.com/)

### 📦 Setup

1. **Clone the Repository**

   ```bash
   git clone https://github.com/Hackonautas/malackathon.git
   cd alveus

2. Setup the Frontend

cd frontend
npm install
npm run dev


3. Setup the Backend

cd ../backend
cargo build --release
cargo run --release


4. Configure Oracle Cloud

Ensure you have your Oracle Cloud credentials and database set up. Update the environment variables accordingly.



🖥️ Usage

🔗 Accessing the Web App

Once the frontend and backend are running, navigate to http://localhost:3000 in your browser to access Alveus.

📊 Exploring Reservoir Data

Dashboard: View current water levels of nearby reservoirs at a glance.

Historical Data: Select a reservoir to view its historical water levels and trends.

Statistics: Access average water levels and other relevant statistics.

Graphs: Interact with dynamic charts to visualize data over time.


🔧 Configuration

📁 Environment Variables

Create a .env file in both frontend and backend directories with the following variables:

Frontend .env

VITE_API_URL=http://localhost:8000/api

Backend .env

DATABASE_URL=oracle://username:password@hostname:port/servicename
PORT=8000

🗄️ Database Setup

1. Provision an Oracle Database

Use the Oracle Cloud Console to create a new database instance.


2. Run Migrations

Navigate to the backend directory and run:

cargo run --release

This will apply the necessary migrations to set up the database schema.



🧑‍💻 Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated.

📋 Guidelines

1. Fork the Repository


2. Create a Branch

git checkout -b feature/YourFeature


3. Commit Your Changes

git commit -m 'Add some feature'


4. Push to the Branch

git push origin feature/YourFeature


5. Open a Pull Request



📜 License

Distributed under the MIT License. See LICENSE for more information.

🙏 Acknowledgements

Svelte

Bulma

Rust

Oracle Cloud
