<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/board/drawing/uniXerr_R50.png"
</p>

###### :warning: If you are working on development part, remember to change the local host(_127.0.0.1_) inside `/etc/hosts/` to `api.unixerr.com` and `tensorboard.api.unixerr.com` for API and TensorBoard server respectively.
###### :warning: Remember to call `/users/add/info` and `/users/add/positions` routes of API server after the classification is done on csv file of input data. 
###### :warning: You can't create an environment if the environment was exported on a different platform than the target machine.
###### :information_source: Both `core` and `server` folders can only be controlled using `controller.py` middleware.

### Setup

* Start an _Apache Cassandra_ server and Fill out _.env_ file with necessary environment variables
* Create an environment: ```conda create -n uniXerr```
* Create the environment using the _uniXerr.yml_ file: ```conda env create -f uniXerr.yml```
* Activate _uniXerr_ environment: ```conda activate uniXerr```
* Update the environment using _uniXerr.yml_ file: ```conda env update -f uniXerr.yml --prune```
* Export your active environment to _uniXerr.yml_ file: ```conda env export | grep -v "^prefix: " > uniXerr.yml```
* Install completion for _typer-cli_: ```typer --install-completion```
* Create a docs file from _uniXerr_ CLI: ```typer app.py utils docs --name uniXerr --output uniXerr-cli.md```

### Usage

```console
$ typer app.py run
Usage: typer run [OPTIONS] COMMAND [ARGS]...

  【  uniXerr CLI controller  】

Options:
  --help  Show this message and exit.

Commands:
  classify-positions
  cluster-positions
  deploy
  develop

$ typer app.py run cluster-positions --help
Usage: typer run cluster-positions [OPTIONS]

Options:
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
Usage: typer run classify-positions [OPTIONS]

Options:
  --csv-path FILE              Path to labeled pc_features csv dataset.
  --input-data-csv-path FILE   Path to input data csv for classification.
  --ddo                        Force deletion with confirmation for dataloader
                               objects.

  --dpm                        Force deletion with confirmation for pre-
                               trained classifier model.

  --epoch INTEGER RANGE        Number of epoch for training classifier.
  --batch-size INTEGER RANGE   Number of batch size for training classifier.
  --device TEXT                Training device. cpu or cuda
  --num-workers INTEGER RANGE  Number of workers for pytroch dataloader
                               object.

  --help                       Show this message and exit.



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

> Running in development mode: __ [API docs](http://api.unixerr.com:8000/docs) 

```console
$ typer app.py run develop --workers 10
```

> Export cassandra table into csv file:

```console
$ cqlsh api.unixerr.com -u username -p password -e "copy unixerr.table_name to '/path/to/table_name.csv' with HEADER = true"
```

> Import exported csv file into cassandra table:

```console
$ cqlsh api.unixerr.com -u username -p password -e "copy unixerr.table_name from '/path/to/table_name.csv' with HEADER = true"
```

> Running in production mode streaming over kafka:

```console
$ typer app.py run deploy --build --kafka
```

> Running TensorBoard for visualization of training and testing DL models:

```console
$ tensorboard --host=tensorboard.unixerr.com --logdir=runs
```

> You can also install uniXerr from pip if you are the commander guy: __ [uniXerr CLI usage](https://github.com/wildonion/uniXerr/blob/master/uniXerr-cli.md)

```console
pip install uniXerr
```
---

# Results

### Position Clustering Process

[Dataloader Object - MinMax Scaler](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_dataloader-DATALOADER.pth)

[Fake Dataset for Offline Training](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_features.csv)

> Plotted Dataset before Clustering using PCA - Standard Scaler
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/server/dataset/pca_pc_beforeClustering.png">
</p>

> Plotted Dataset before Clustering using TSNE - Standard Scaler
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/server/dataset/tsne_pc_beforeClustering.png">
</p>
    
[Clustered Dataset Based on Latent Space of Pre-trained VAE model](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_features_labeled-latent.csv)

[Clustered Dataset Based on Position Clustering data](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_features_labeled-raw.csv)

[VAE Pre-trained Model - Normal PDF](https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/pc_model_vae.pth)

> Clusters Found by KMeans on Latent Space of Pre-trained VAE model
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/clusters-kmeans-latent.png">
</p>

> Clusters Found by KMeans on Position Clustering Dataset - Plotted using PCA | Standard Scaler
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/clusters-kmeans-pca-raw.png">
</p>

> Clusters Found by KMeans on Position Clustering Dataset - Plotted using TSNE | Standard Scaler
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/clusters-kmeans-tsne-raw.png">
</p>

> VAE Model Training Loss 
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/pc_model_loss.png">
</p>

### Position Classification Process

[Training Dataloader Object of Clustered Dataset Based on Latent Space of Pre-trained VAE model](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_features_labeled_training_tensors-latent-DATALOADER.pth)

[Testing Dataloader Object of Clustered Dataset Based on Latent Space of Pre-trained VAE model](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_features_labeled_testing_tensors-latent-DATALOADER.pth)

[Training Dataloader Object of Clustered Dataset Based on Position Clustering data](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_features_labeled_training_tensors-raw-DATALOADER.pth)

[Testing Dataloader Object of Clustered Dataset Based on Position Clustering data](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_features_labeled_testing_tensors-raw-DATALOADER.pth)

> Percentage of Positions before Classification on Clustered Dataset Based on Latent Space of Pre-trained VAE model
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/server/dataset/pp_pc_beforeClassification-latent.png">
</p>

> Percentage of Positions before Classification on Clustered Dataset Based on Position Clustering data
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/server/dataset/pp_pc_beforeClassification-raw.png">
</p>

[Classifier Pre-trained Model - Trained and Tested on Clustered Dataset Based on Latent Space of Pre-trained VAE model](https://github.com/wildonion/uniXerr/blob/master/core/position_classification/utils/pc_model_classifier-latent.pth)

[Classifier Pre-trained Model - Trained and Tested on Clustered Dataset Based on Position Clustering data](https://github.com/wildonion/uniXerr/blob/master/core/position_classification/utils/pc_model_classifier-raw.pth)

> Classifier Model Training Accuracy - Clustered Dataset Based on Latent Space of Pre-trained VAE model
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_classification/utils/pc_model_training_acc-latent.png">
</p>

> Classifier Model Testing Accuracy - Clustered Dataset Based on Latent Space of Pre-trained VAE model
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_classification/utils/pc_model_testing_acc-latent.png">
</p>

> Classifier Model Training Loss - Clustered Dataset Based on Latent Space of Pre-trained VAE model
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_classification/utils/pc_model_training_loss-latent.png">
</p>

> Classifier Model Training Accuracy - Clustered Dataset Based on Position Clustering data
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_classification/utils/pc_model_training_acc-raw.png">
</p>

> Classifier Model Testing Accuracy - Clustered Dataset Based on Position Clustering data
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_classification/utils/pc_model_testing_acc-raw.png">
</p>

> Classifier Model Training Loss - Clustered Dataset Based on Position Clustering data
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_classification/utils/pc_model_training_loss-raw.png">
</p>

[Classification Results on Arbitrary Inputs - Classified using Pre-trained Model of Clustered Dataset Based on Latent Space of Pre-trained VAE model and Clustered Dataset Based on Position Clustering data](https://github.com/wildonion/uniXerr/blob/master/server/db/_imported/)
