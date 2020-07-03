<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/board/drawing/uniXerr_R50.png"
</p>

###### :warning: You can't create an environment if the environment was exported on a different platform than the target machine.
###### :information_source: `uniXerr.yml` was exported on Ubuntu Linux 20.04 LTS.
###### :information_source: Both `core` and `server` folders can only be controlled using `controller.py` middleware.

### Setup

* Create the environment with the latest version of python: ```conda create -n uniXerr python=3```
* Activate _uniXerr_ environment: ```conda activate uniXerr```
* Update the environment using _uniXerr.yml_ file: ```conda env update -f uniXerr.yml --prune```
* Export your active environment to _uniXerr.yml_ file: ```conda env export | grep -v "^prefix: " > uniXerr.yml```
* Install completion for _typer-cli_: ```typer --install-completion```
* Create a docs file from _uniXerr_ CLI: ```typer app.py utils docs --name uniXerr --output uniXerr-cli.md```

### Usage

```console
$ typer app.py run
Usage: typer run [OPTIONS] COMMAND [ARGS]...

  |> uniXerr CLI controller <|

Options:
  --help  Show this message and exit.

Commands:
  classify-positions
  cluster-positions
  deploy
  develop

$ typer app.py run cluster-positions --help
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

  --cluster-on-raw-data        Clustering on pc_features dataset, default is
                               set to VAE latent space
  --cluster-method TEXT        Clustering method. kmeans or hdbscan; hdbscan
                               is not suitable for latent space of VAE and has
                               some drawbacks for new dataset.

  --plot-method TEXT           Plotting method for data. pca or tsne; if you
                               want plot data before clustering on different
                               methods just remove the pc_dataloader.pth with
                               --ddo option.

  --help                       Show this message and exit.

$ typer app.py run classify-positions --help
Usage: app.py run classify-positions [OPTIONS]

Options:
  --csv-path FILE  Path to labeled pc_features csv dataset.
  --help           Show this message and exit.

$ typer app.py run deploy --help
Usage: app.py run deploy [OPTIONS]

Options:
  --build  Building for production.
  --kafka  Streamer processor for online training.
  --help   Show this message and exit.

$ typer app.py run develop --help
Usage: app.py run develop [OPTIONS]

Options:
  --workers INTEGER RANGE  Number of workers
  --help                   Show this message and exit.
```

> Running in development mode:

```console
$ typer app.py run develop --workers 10
```

> Running in production mode streaming over kafka:

```console
$ typer app.py run deploy --build --kafka
```
> You can also install uniXerr from pip if you are the commander guy: __ [uniXerr CLI usage](https://github.com/wildonion/uniXerr/blob/master/uniXerr-cli.md)

```console
pip install uniXerr
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
    
[Clustered Dataset Based on Latent Space of Pre-Trained VAE model](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_features_labeled-latent.csv)

[Clustered Dataset Based on Position Clustering data](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_features_labeled-raw.csv)

[VAE Pre-Trained Model - Normal PDF](https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/pc_model.pth)

> Clusters Found by KMeans on Latent Space of Pre-Trained VAE model
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/clusters-kmeans-latent.png"
</p>

> Clusters Found by KMeans on Position Clustering Dataset - Plotted using PCA | Standard Scaler
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/clusters-kmeans-pca-raw.png"
</p>

> Clusters Found by KMeans on Position Clustering Dataset - Plotted using TSNE | Standard Scaler
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/clusters-kmeans-tsne-raw.png"
</p>

> VAE Model Training Loss 
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/pc_model_loss.png"
</p>


