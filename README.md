<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/board/drawing/uniXerr_R50.png"
</p>

### Setup

* Create the environment from _uniXerr.yml_ file: ```conda env create -f uniXerr.yml```
* Activate _uniXerr_ environment: ```conda activate uniXerr```
* Update the environment using _uniXerr.yml_ file: ```conda env update -f uniXerr.yml --prune```
* Export your active environment to _uniXerr.yml_ file: ```conda env export | grep -v "^prefix: " > uniXerr.yml```
* Install completion for _typer-cli_: ```typer --install-completion```

### Usage

> Both `core` and `server` folders can only be controlled using `controller.py` middleware.

```console
$ python app.py
Usage: app.py [OPTIONS] COMMAND [ARGS]...

  uniXerr CLI user manager.

Options:
  --install-completion  Install completion for the current shell.
  --show-completion     Show completion for the current shell, to copy it or
                        customize the installation.

  --help                Show this message and exit.

Commands:
  classify-positions
  cluster-positions
  deploy
  develop

$ python app.py cluster-positions --help
Usage: app.py cluster-positions [OPTIONS]

Options:
  --training TEXT              Training algorithm. offline or online; offline
                               uses a csv file and online uses a streamer for
                               training.

  --generate-fake-samples      Generating fake samples for training.
  --epoch INTEGER RANGE        Number of epoch for training VAE.
  --batch-size INTEGER RANGE   Number of batch size for training VAE.
  --device TEXT                Training device. cpu or cuda
  --num-workers INTEGER RANGE  Number of workers for pytroch dataloader
                               object.

  --latent-dim INTEGER RANGE   Dimension of VAE latent space.
  --ddo                        Force deletion with confirmation for dataloader
                               object.

  --dpm                        Force deletion with confirmation for pre-
                               trained VAE model.

  --cluster-on-raw-data        Clustering on pc_features dataset.
  --cluster-method TEXT        Clustering method. kmeans or hdbscan; hdbscan
                               is not suitable for latent space of VAE and has
                               some drawbacks for new dataset.

  --plot-method TEXT           Plotting method for data. pca or tsne; if you
                               want plot data before clustering on different
                               methods just remove the pc_dataloader.pth with
                               --ddo option.

  --help                       Show this message and exit.

$ python app.py classify-positions --help
Usage: app.py classify-positions [OPTIONS]

Options:
  --csv-path FILE  Path to labeled pc_features csv dataset.
  --help           Show this message and exit.

$ python app.py deploy --help
Usage: app.py deploy [OPTIONS]

Options:
  --build  Building for production.
  --help   Show this message and exit.

$ python app.py develop --help
Usage: app.py develop [OPTIONS]

Options:
  --workers INTEGER RANGE  Number of workers
  --asgi-server TEXT       ASGI server.
  --help                   Show this message and exit.
```

> Running in development mode:

```console
$ python app.py develop --asgi-server uvicorn --workers 4
```

> Running in production mode:

```console
$ python app.py deploy --build
```

---

# Results

### Position Clustering Process

[Dataloader Object - MinMax Scaler](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_dataloader.pth)

[Fake Dataset for Offline Training](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_features.csv)

> Plotted Dataset before Clustering using PCA - Standard Scaler
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/server/dataset/pca_pc_beforeClustering.png"
</p>

> Plotted Dataset before Clustering using TSNE - Standard Scaler
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/server/dataset/tsne_pc_beforeClustering.png"
</p>
    
#### 

[Clustered Dataset](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_features_labeled.csv)

[VAE Pre-Trained Model](https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/pc_model.pth)

> Clusters Found by KMeans on Latent Space of Pre-Trained VAE model - Plotted using PCA
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/clusters-kmeans-pca.png"
</p>

> Clusters Found by KMeans on Latent Space of Pre-Trained VAE model - Plotted using TSNE
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/clusters-kmeans-tsne.png"
</p>

> VAE Model Training Loss 
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/pc_model_loss.png"
</p>


