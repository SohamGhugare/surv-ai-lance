# Surv-AI-Lance - A project by Team Omniscience

## Components

### AI Model

The AI Model is based on a novel Time Distributed ResNet101V2 image classification model approach that is then trained using transfer learning along with LSTM layers to classify the videos and saving the temporal and spatial locality streams. This trained model then takes the input from the user and then checks chunks of frames to classify the video into the respective categories (Violence, Weaponized Violence, Normal).

### CDN - Content Delivery Network

**(Hosted on AWS)** This basically converts the static files into a dynamic file and then serves it to the user. This is done to convert the files into links for easy broadcasting.

### Main Server

A communication server for fetching nearest 5 police stations from hosted MySQL database and sending structured alerts to the police stations using **WebSockets**.

### Web Application

A WebSocket based webapp which constantly maintains a connection with the main server using websockets and responds to alerts.

### Blockchain

Our own in-house blockchain to store the video data and the classification results. This is done to ensure the integrity of the data and to prevent any tampering with the data.
