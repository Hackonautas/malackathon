# 🌊 Alveus - Reservoir Data Dashboard

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Technology](https://img.shields.io/badge/Tech-Svelte%20|%20Bulma%20|%20Rust-blue)
![Cloud](https://img.shields.io/badge/Cloud-Oracle%20Cloud-orange)

## 📜 Table of Contents

- [🌊 Alveus - Reservoir Data Dashboard](#-alveus---reservoir-data-dashboard)
  - [📖 Table of Contents](#-table-of-contents)
  - [🚀 About the Project](#-about-the-project)
  - - [📝 Project Report](#-project-report)
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


## 📝 Project Report

The project began at 10:00 AM after attending a presentation from the various partners at the Malakathon hackathon. We started by dividing the work among the five team members, ensuring efficiency and distribution based on each individual’s expertise. This approach allowed everyone to contribute and learn, ensuring that we leveraged each member’s strengths.

In the days prior to the hackathon, we experimented with Oracle Cloud, setting up an autonomous database. This system's automatic detection of data types facilitated both the usage and the creation of our project. We also created a static IP for our instance, exploring the different cloud options available for instance configuration. After preparing a cost estimate, we were ready for the hackathon day.

On the day of the event, we divided into two groups: three team members focused on the frontend and two on the backend.

Frontend

The frontend team started by sketching the design on paper, which was later processed using GPT-4’s computer vision capabilities. This allowed us to automatically interpret the drawings and add nuanced suggestions for improvement. We used these prompts within Vercel V0 Chat, which helped accelerate the development of the frontend. We then implemented the final design using Svelte and Bulma, ensuring a clean and responsive user interface. Prior to coding, we also conducted a brainstorming session to map out the structure of the page.

Backend

For the backend, we opted to use Rust to develop a solution that prioritized performance and security. Rust enabled us to write efficient and optimized code while maintaining high security standards.

Conclusion

The project involved long hours of hard work, but with the support from professionals who visited during the event and the cooperation among team members, we successfully delivered the project.

This experience has been enriching, allowing us to push the limits of what we could achieve together while benefiting from the tools and resources at our disposal.


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

2. **Setup the Frontend**
```
cd frontend
npm install
npm run dev

```
3. **Setup the Backend**
```
cd ../backend
cargo build --release
cargo run --release
```

4. **Configure Oracle Cloud**

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
```
git checkout -b feature/YourFeature
```

3. Commit Your Changes
```
git commit -m 'Add some feature'
```

4. Push to the Branch
```
git push origin feature/YourFeature
```

5. Open a Pull Request



📜 License

Distributed under the MIT License. See LICENSE for more information.

🙏 Acknowledgements

Svelte

Bulma

Rust

Oracle Cloud
