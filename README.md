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
    - [🎨 Color Palette: 🟦 🟪 ⚪️ 🟫](#-🎨-color-palette-🟦-🟪-⚪️-🟫)
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

---

The project officially began at 10:00 AM after we attended a presentation by various partners at the Malakathon hackathon. We immediately organized our team of five, strategically dividing the tasks based on each member’s strengths and expertise. This not only ensured that the workload was distributed efficiently but also fostered an environment where everyone could contribute meaningfully and learn from the process.

In the days leading up to the hackathon, we conducted several trials with Oracle Cloud, where we set up an autonomous database. The system's ability to automatically detect data types streamlined both the implementation and the management of our project. Additionally, we assigned a static IP address to our instance and explored different cloud configuration options to optimize performance. After finalizing a cost estimate, we felt confident and prepared for the main event.

On the day of the hackathon, we split into two groups: three members focused on the frontend while the other two concentrated on the backend.

**Frontend Development**

Initially, we aimed to use React, driven by its popularity and the wide range of resources available. However, as we delved into the implementation, we realized that our limited experience with React, combined with the complexity of the framework, made it difficult to deliver a high-quality solution within the tight time constraints. After evaluating the situation, we decided to pivot and opted for Svelte and Bulma instead. These technologies allowed us to work more efficiently and achieve our goals without compromising on design or functionality.

We began by sketching the design by hand, which we later processed using GPT-4's computer vision capabilities. This innovative approach enabled us to automatically interpret the sketches and receive nuanced suggestions for improvement. These recommendations were integrated into Vercel V0 Chat, which further accelerated the frontend development process. Once we had a solid design plan, we implemented it using Svelte and Bulma, ensuring a clean, responsive, and aesthetically pleasing user interface. Before diving into the coding, we held a brainstorming session to carefully map out the structure and flow of the page, ensuring a cohesive and user-friendly experience.

<<<<<<< HEAD

---

**Backend of the "Alveus" Project**

This backend is responsible for managing and processing data related to reservoirs and their water volumes, with the goal of providing updated information on the water status of various reservoirs through both historical and real-time data. The data is structured across three main files, each containing complementary information: reservoirs, water, and a reservoir listing.

- **Technology Stack**

For the backend, we chose to work with Rust, prioritizing performance and security. Rust's emphasis on memory safety without sacrificing speed allowed us to write highly optimized code. The language's inherent security features provided peace of mind, knowing that our solution would be both robust and efficient. We integrated the backend seamlessly with Oracle Cloud, ensuring that data handling was smooth and secure throughout the entire project lifecycle.

- **Data Loading**

To ensure scalability and facilitate the handling of large volumes of data, we used SQL Developer to load and manipulate data from the three primary files. This tool allows us to manage large tables and optimize queries.

- **Data Structure**

The following are descriptions of the three datasets used in the project:

 Reservoir Data
```
ID: Unique identifier for each reservoir.

AMBITO_NOMBRE: Name of the hydrographic basin or region to which the reservoir belongs.

EMBALSE_NOMBRE: Name of the reservoir.

AGUA_TOTAL: Total volume of water stored in the reservoir (in cubic hectometers).

ELECTRICO_FLAG: Binary indicator for whether the reservoir is used for electricity generation (0 = Not used, 1 = Used for electricity generation).
```

 Water Data
```
FECHA: Date and time of the water measurement in the reservoir.

AGUA_ACTUAL: Current volume of water stored in the reservoir on the specified date (in cubic hectometers).

ID: Unique identifier linking the measurement to the corresponding reservoir.
```

 Reservoir Listing
```
CODIGO: Unique code for the reservoir.

NOMBRE: Official name of the reservoir or dam.

EMBALSE: Alternative or common name of the reservoir.

X: X coordinate (longitude) of the reservoir.

Y: Y coordinate (latitude) of the reservoir.

DEMARC: Hydrographic demarcation to which the reservoir belongs.

CAUCE: Name of the watercourse or river where the reservoir is located.

GOOGLE: Google Maps search link for the reservoir.

OPENSTREETMAP: OpenStreetMap link to the reservoir.

WIKIDATA: Wikidata link to the reservoir.

PROVINCIA: Province in which the reservoir is located.

CCAA: Autonomous Community in which the reservoir is located.

TIPO: Type of dam (e.g., loose materials, concrete, etc.).

COTA_CORON: Elevation of the dam crest (in meters above sea level).

ALT_CIMIEN: Foundation height or maximum dam height (in meters).

INFORME: Link to a report or web service related to the reservoir.
```

- **Data Processing**

**Duplicate Row Removal**: During the data cleaning process, duplicate rows and NaN values were removed. In case of duplicates, the row with the most available data was retained.

**Statistical Functions**: Functions were implemented to retrieve maximum, minimum, historical, and average values of the water volume stored in each reservoir.

**Temporal Selection**: We added the ability to select data based on specific time ranges to analyze changes in water storage volumes over time.


- **Table Joining**

Finally, we joined the tables for reservoirs, water, and the reservoir listing using the ID common to all of them. This allowed us to combine geographic and technical data for each reservoir with historical water measurements, providing a complete view of the status and capacity of the reservoirs.


---

=======
**Backend Development**

For the backend, we chose to work with Rust, prioritizing performance and security. Rust's emphasis on memory safety without sacrificing speed allowed us to write highly optimized code. The language's inherent security features also provided peace of mind, knowing that our solution would be both robust and efficient. We integrated the backend seamlessly with Oracle Cloud, ensuring that data handling was smooth and secure throughout the entire project lifecycle.
>>>>>>> frontend

**Challenges and Solutions**

One of the most significant challenges we faced was the shift in our frontend technology stack. While React and daisyui is a powerful tool, its steep learning curve and the complexity of setting it up in such a short timeframe posed issues that slowed our initial progress. After recognizing this, switching to Svelte and Bulma proved to be a game-changer, as it allowed us to regain momentum and work in a more straightforward, adaptable environment. Another challenge was ensuring that the backend architecture, designed in Rust, could efficiently communicate with the frontend and handle cloud interactions with Oracle. However, by leveraging the strengths of each team member, we were able to overcome these obstacles effectively.

**Conclusion**

Throughout the project, we worked long hours, but the support from professionals at the event and the collaboration within our team were key to our success. We were able to deliver a fully functional solution that not only met the project requirements but also exceeded our initial expectations.

This hackathon experience has been incredibly rewarding. It allowed us to push the boundaries of our capabilities, work with cutting-edge tools, and adapt quickly to unforeseen challenges. Most importantly, it reinforced the value of teamwork and adaptability when working under pressure, showing us how far we could go by leveraging the skills and resources at our disposal.

---

### 🔍 Features

- **Real-Time Data:** Access up-to-date information on water levels of nearby reservoirs.
- **Historical Data:** View and analyze past water levels to identify trends.
- **Averages & Statistics:** Get average water levels and other relevant statistics.
- **Interactive Graphs:** Visualize data through dynamic and responsive charts.
- **Responsive Design:** Built with Bulma to ensure compatibility across devices.
- **High Performance:** Backend developed in Rust for speed and efficiency.
- **Modern Frontend:** Utilizes Svelte for a reactive and seamless user experience.
- **Secure & Scalable:** Hosted on Oracle Cloud for reliability and scalability.
---
### 🛠️ Technologies Used

- **Frontend:**
  - [Svelte](https://svelte.dev/) - A radical new approach to building user interfaces.
  - [Bulma](https://bulma.io/) - A modern CSS framework based on Flexbox.
  
- **Backend:**
  - [Rust](https://www.rust-lang.org/) - A fast and memory-efficient programming language.
  
- **Database & Hosting:**
  - [Oracle Cloud](https://www.oracle.com/cloud/) - Comprehensive cloud services for databases and hosting.
---

## 🎨 Color Palette: 🟦 🟪 ⚪️ 🟫

In the development of our website, we carefully selected a color palette focused on accessibility for individuals with visual impairments. Our goal is to ensure that all users can easily navigate and comprehend the content without difficulty. The combination of colors we chose follows the principles outlined by the WCAG (Web Content Accessibility Guidelines) to maximize readability and contrast.

**Selected Colors**:

Dark blue and bluish-gray tones: The dominant colors are dark blue and grayish shades, which provide adequate contrast without being too harsh on the eyes. These colors allow text to be easily readable by individuals with low vision or color blindness, while also minimizing visual fatigue.

Soft cream and off-white tones: The lighter sections of the design, combining soft cream with off-white, are designed to avoid overly bright backgrounds. This is particularly important for users with light sensitivity or visual impairments that make it difficult to read against pure white or highly contrasted backgrounds.


**Accessibility Principles Applied**:

1. **Adequate contrast**: The selected color scheme ensures sufficient contrast between text and background, in line with WCAG recommendations, making the content legible for users with visual disabilities, including color blindness.


2. **Ease of reading**: The chosen tones allow for easy distinction between different sections of the site without overwhelming users. We avoided saturated or neon colors, which could be distracting or difficult to perceive for users with visual impairments.


3. **Visual consistency**: The color palette provides a cohesive and relaxing visual experience, reducing cognitive effort required to process the content.



With this approach, our website becomes a more inclusive tool, offering all users, regardless of their abilities, an accessible and enjoyable experience.


---
## 🌐 Live Demo

Check out the live version of **Alveus** [here](https://your-live-demo-link.com).

## 💾 Installation

### ⚙️ Prerequisites

Before you begin, ensure you have met the following requirements:

- **Node.js** (v14 or later) installed. [Download Node.js](https://nodejs.org/)
- **Rust** installed. [Install Rust](https://www.rust-lang.org/tools/install)
- An **Oracle Cloud** account with access to set up databases and instances.
- **Git** installed. [Download Git](https://git-scm.com/)
---
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


---
🖥️ Usage

🔗 Accessing the Web App

Once the frontend and backend are running, navigate to http://localhost:3000 in your browser to access Alveus.

---

📊 Exploring Reservoir Data

- **Dashboard**: View current water levels of nearby reservoirs at a glance.

- **Historical Data**: Select a reservoir to view its historical water levels and trends.

- **Statistics**: Access average water levels and other relevant statistics.

 - **Graphs**: Interact with dynamic charts to visualize data over time.
---
Here's an improved version of the section with a clearer structure and explanation:


---

🔧 Configuration Setup

📁 Environment Variables

You will need to create a .env file in both the frontend and backend directories to store the required environment variables.
```
Frontend .env File:

VITE_API_URL=http://localhost:8000/api

This variable sets the API URL that the frontend will use to communicate with the backend.

Backend .env File:

DATABASE_URL=oracle://username:password@hostname:port/servicename
PORT=8000

DATABASE_URL: Specifies the connection string to your Oracle database instance, including credentials and the service name.

PORT: Defines the port on which the backend will run (in this case, 8000).
```

🗄️ Database Setup

1. Provision an Oracle Database



Log into the Oracle Cloud Console and create a new database instance. Make sure to note down the connection details, which you will include in the DATABASE_URL environment variable.

2. Run Migrations



Once your database is set up, navigate to the backend directory in your project and run the following command to apply the necessary migrations and set up the database schema:

cargo run --release

This command will execute the migrations required for initializing your database structure and ensure that your application is ready to interact with the database.

---
🧑‍💻 Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated.

📋 Guidelines

**1. Fork the Repository**


**2. Create a Branch**
```
git checkout -b feature/YourFeature
```

**3. Commit Your Changes**
```
git commit -m 'Add some feature'
```

**4. Push to the Branch**
```
git push origin feature/YourFeature
```

**5. Open a Pull Request**



📜 License

Distributed under the MIT License. See LICENSE for more information.

🙏 Acknowledgements

Svelte

Bulma

Rust

Oracle Cloud
