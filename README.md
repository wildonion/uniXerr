<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/board/drawing/uniXerr_R50.png"
</p>

# uniXerr Skeleton Development Setup

### Requirements

* **Remember that _auth_ (_suproxy_) microservice is responsible for handling all CRUD operations related to _postgres_ (_cassandra_)**

* **Install _rustup_, _pm2_, _postgres_, _cassandra_ and _kafka_**

* **Install prerequisite packages on Linux:** ```sudo apt install openssl libssl-dev cmake libpq-dev```

* **Install _openssl_ for _diesel_ using ```choco install openssl```, _cmake_ for _rdkafka_ lib using ```choco install cmake``` and _gcc_/_g++_ with _mingw_ using ```choco install mingw``` on Windows** 

* **Put _postgres_ lib and bin path into environment variable on Windows:** ```C:\Program Files\PostgreSQL\13\lib``` and ```C:\Program Files\PostgreSQL\13\bin```

* **Install _cargo_ bullshits:** ```cargo install diesel_cli --no-default-features --features postgres && cargo install systemfd cargo-watch```  

### Updating `auth` Microservice API Acess Level

* **Updating access level to admin access:** ```cd server/skeleton/microservices/auth/ && cargo run <USERNAME> <ACCESS_LEVEL>```
    * **eg - change access level of user _wildonion_ to admin level:** ```cd server/skeleton/microservices/auth/ && cargo run wildonion 2```

### Running Microservices Commands

* **Run _auth_ microservice using one the following commands:** 
    * ```systemfd --no-pid -s http::7366 -- cargo watch -C server/skeleton/microservices/auth -x run```
    * ```cargo watch -C server/skeleton/microservices/auth -x run```

* **Run _suproxy_ load balancer using one the following commands:**
    * ```systemfd --no-pid -s http::7368 -- cargo watch -C server/skeleton/microservices/suproxy -x run```
    * ```cargo watch -C server/skeleton/microservices/suproxy -x run```

* **Run _tracer_ microservice using one the following commands:**
    * ```systemfd --no-pid -s http::7363 -- cargo watch -C server/skeleton/microservices/tracer -x run```
    * ```cargo watch -C server/skeleton/microservices/tracer -x run```

* **Run _coiniXerr_ network:**
    * ```cargo watch -C server/skeleton/microservices/coiniXerr -x run```

# uniXerr Skeleton Production Setup

### Setup Postgres DB and User

```
CREATE DATABASE uniXerr;
CREATE USER uniXerr WITH ENCRYPTED PASSWORD 'uniXerr';
GRANT ALL PRIVILEGES ON DATABASE uniXerr TO uniXerr;
ALTER USER uniXerr WITH SUPERUSER;
```

* **Build & run each microservice:** ```sudo chmod +x deploy.sh && ./deploy.sh```

### uniXerr Skeleton Postgres Database Setup

* **Generate _migrations_ folder, create uniXerr postgres db, `diesel.toml` file on first run or run existing migrations into the database:** 

    * ```cd server/skeleton/microservices && diesel setup --migration-dir server/skeleton/microservices/auth/migrations/```

* **Generate SQL files for your table operations:** ```diesel migration generate SQL-OPERATION_TABLE-NAME```

    * **eg - create users table for _auth_ microservice:** ```diesel migration generate create_users --migration-dir server/skeleton/microservices/auth/migrations/```

* **Migrate tables into postgres db and generate(update) `schema.rs` file inside _src_ folder:** ```diesel migration run```

    * **eg - migrate all SQL files of operations of _auth_ microservice into the database:** ```diesel migration run --migration-dir server/skeleton/microservices/auth/migrations/```
    * **note - in order to generate the `schema.rs` in _src_ folder the ```diesel migration run``` command must have a successful result**
    * **note - you can also create sql files (`up.sql` and `down.sql`) for your table in each migrations folder by hand then run the ```diesel setup``` command to migrate them all into the db at once**
    * **note - down migration command for each table is: ```diesel migration down```**

* **Check diesel migrations errors:** ```diesel migration list```

    * **eg - check migrations errors for _auth_ microservice:** ```diesel migration list --migration-dir server/skeleton/microservices/auth/migrations/```

# AI Core Development Guide

###### ⚠️ If you are working on development part, remember to change the local host(_127.0.0.1_) inside `/etc/hosts/` to `api.unixerr.com` and `tensorboard.api.unixerr.com` for API and TensorBoard server respectively.
###### ⚠️ Remember to call `/users/add/info` and `/users/add/positions` routes of API server after the classification is done on csv file of input data. 
###### ⚠️ You can't create an environment if the environment was exported on a different platform than the target machine.
###### ❗️ Both `core` and `server` folders can only be controlled using `controller.py` middleware.

### Setup

* Start an _Apache Cassandra_ server and fill out _.env_ file with necessary variables
* Create an environment with a specific python version: ```conda create -n uniXerr python=3.8```
* Create the environment using the _uniXerr.yml_ file: ```conda env create -f uniXerr.yml```
* Activate _uniXerr_ environment: ```conda activate uniXerr```
* Update the environment using _uniXerr.yml_ file: ```conda env update -f uniXerr.yml --prune```
* Export your active environment to _uniXerr.yml_ file: ```conda env export | grep -v "^prefix: " > uniXerr.yml```
* Install _pm2_: ```wget -qO- https://getpm2.com/install.sh | bash```
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

### 📌 Position Clustering Process

[Dataloader Object - MinMax Scaler](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_dataloader-DATALOADER.pth)

[Fake Dataset for Offline Training](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_features.csv)

> 📊 Plotted Dataset before Clustering using PCA - Standard Scaler
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/server/dataset/pca_pc_beforeClustering.png">
</p>

> 📊 Plotted Dataset before Clustering using TSNE - Standard Scaler
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/server/dataset/tsne_pc_beforeClustering.png">
</p>
    
[Clustered Dataset Based on Latent Space of Pre-trained VAE model](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_features_labeled-latent.csv)

[Clustered Dataset Based on Position Clustering data](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_features_labeled-raw.csv)

[VAE Pre-trained Model - Normal PDF](https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/pc_model_vae.pth)

> 📊 Clusters Found by KMeans on Latent Space of Pre-trained VAE model
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/clusters-kmeans-latent.png">
</p>

> 📊 Clusters Found by KMeans on Position Clustering Dataset - Plotted using PCA | Standard Scaler
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/clusters-kmeans-pca-raw.png">
</p>

> 📊 Clusters Found by KMeans on Position Clustering Dataset - Plotted using TSNE | Standard Scaler
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/clusters-kmeans-tsne-raw.png">
</p>

> 📊 VAE Model Training Loss 
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_clustering/utils/pc_model_loss.png">
</p>

### 📌 Position Classification Process

[Training Dataloader Object of Clustered Dataset Based on Latent Space of Pre-trained VAE model](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_features_labeled_training_tensors-latent-DATALOADER.pth)

[Testing Dataloader Object of Clustered Dataset Based on Latent Space of Pre-trained VAE model](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_features_labeled_testing_tensors-latent-DATALOADER.pth)

[Training Dataloader Object of Clustered Dataset Based on Position Clustering data](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_features_labeled_training_tensors-raw-DATALOADER.pth)

[Testing Dataloader Object of Clustered Dataset Based on Position Clustering data](https://github.com/wildonion/uniXerr/blob/master/server/dataset/pc_features_labeled_testing_tensors-raw-DATALOADER.pth)

> 📊 Percentage of Positions before Classification on Clustered Dataset Based on Latent Space of Pre-trained VAE model
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/server/dataset/pp_pc_beforeClassification-latent.png">
</p>

> 📊 Percentage of Positions before Classification on Clustered Dataset Based on Position Clustering data
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/server/dataset/pp_pc_beforeClassification-raw.png">
</p>

[Classifier Pre-trained Model - Trained and Tested on Clustered Dataset Based on Latent Space of Pre-trained VAE model](https://github.com/wildonion/uniXerr/blob/master/core/position_classification/utils/pc_model_classifier-latent.pth)

[Classifier Pre-trained Model - Trained and Tested on Clustered Dataset Based on Position Clustering data](https://github.com/wildonion/uniXerr/blob/master/core/position_classification/utils/pc_model_classifier-raw.pth)

> 📊 Classifier Model Training Accuracy - Clustered Dataset Based on Latent Space of Pre-trained VAE model
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_classification/utils/pc_model_training_acc-latent.png">
</p>

> 📊 Classifier Model Testing Accuracy - Clustered Dataset Based on Latent Space of Pre-trained VAE model
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_classification/utils/pc_model_testing_acc-latent.png">
</p>

> 📊 Classifier Model Training Loss - Clustered Dataset Based on Latent Space of Pre-trained VAE model
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_classification/utils/pc_model_training_loss-latent.png">
</p>

> 📊 Classifier Model Training Accuracy - Clustered Dataset Based on Position Clustering data
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_classification/utils/pc_model_training_acc-raw.png">
</p>

> 📊 Classifier Model Testing Accuracy - Clustered Dataset Based on Position Clustering data
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_classification/utils/pc_model_testing_acc-raw.png">
</p>

> 📊 Classifier Model Training Loss - Clustered Dataset Based on Position Clustering data
<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/core/position_classification/utils/pc_model_training_loss-raw.png">
</p>

[Classification Results on Arbitrary Inputs - Classified using Pre-trained Model of Clustered Dataset Based on Latent Space of Pre-trained VAE model and Clustered Dataset Based on Position Clustering data](https://github.com/wildonion/uniXerr/blob/master/server/db/_imported/)
