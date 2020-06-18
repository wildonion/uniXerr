<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/board/drawing/uniXerr_R50.png"
</p>

# Setup
* Install pm2: ```npm install pm2@latest -g```
* Activate _uniXerr_ environment: ```conda activate uniXerr```
* Create the environment from the _uniXerr.yml_ file: ```conda env create -f uniXerr.yml```
* Update the environment using the _uniXerr.yml_ file: ```conda env update -f uniXerr.yml --prune```
* Export your active environment to _uniXerr.yml_ file: ```conda env export > uniXerr.yml```

# Usage
> `pm2 start app.py`

This is the main server of the uniXerr protocol and it can be controlled using `eye.py` through `ZMQ` socket. It's a layer on top of the uniXerr core.

---

# Position Clustering Algorithm

#### Prerequisites
[Dataloader Object - MinMax Scaler](https://github.com/wildonion/uniXerr/blob/master/core/server/streamer/dataset/pc_dataloader.pth)

[Fake Dataset for Offline Training](https://github.com/wildonion/uniXerr/blob/master/core/server/streamer/dataset/pc_features.csv)

> Plotted Dataset before Clustering using PCA - Standard Scaler
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/server/streamer/dataset/pca_pc_beforeClustering.png"
</p>

> Plotted Dataset before Clustering using TSNE - Standard Scaler
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/server/streamer/dataset/tsne_pc_beforeClustering.png"
</p>
    
#### Results

[Clustered Dataset](https://github.com/wildonion/uniXerr/blob/master/core/kernel/position_clustering/utils/pc_features_labeled.csv)

[VAE Pre-Trained Model](https://github.com/wildonion/uniXerr/blob/master/core/kernel/position_clustering/utils/pc_model.pth)

> Clusters Found by KMeans on Latent Space of Pre-Trained VAE model - Plotted using PCA
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/kernel/position_clustering/utils/clusters-kmeans-pca.png"
</p>

> Clusters Found by KMeans on Latent Space of Pre-Trained VAE model - Plotted using TSNE
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/kernel/position_clustering/utils/clusters-kmeans-tsne.png"
</p>

> VAE Model Training Loss 
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/kernel/position_clustering/utils/pc_model_loss.png"
</p>


