# `uniXerr`

【  uniXerr CLI controller  】

**Usage**:

```console
$ uniXerr [OPTIONS] COMMAND [ARGS]...
```

**Options**:

* `--help`: Show this message and exit.

**Commands**:

* `classify-positions`
* `cluster-positions`
* `deploy`
* `develop`

## `uniXerr classify-positions`

**Usage**:

```console
$ uniXerr classify-positions [OPTIONS]
```

**Options**:

* `--csv-path FILE`: Path to labeled pc_features csv dataset.
* `--input-data-csv-path FILE`: Path to input data csv for classification.
* `--ddo`: Force deletion with confirmation for dataloader objects.
* `--dpm`: Force deletion with confirmation for pre-trained classifier model.
* `--epoch INTEGER RANGE`: Number of epoch for training classifier.
* `--batch-size INTEGER RANGE`: Number of batch size for training classifier.
* `--device TEXT`: Training device. cpu or cuda
* `--num-workers INTEGER RANGE`: Number of workers for pytroch dataloader object.
* `--help`: Show this message and exit.

## `uniXerr cluster-positions`

**Usage**:

```console
$ uniXerr cluster-positions [OPTIONS]
```

**Options**:

* `--generate-fake-samples`: Generating fake samples for training.
* `--epoch INTEGER RANGE`: Number of epoch for training VAE.
* `--batch-size INTEGER RANGE`: Number of batch size for training VAE.
* `--device TEXT`: Training device. cpu or cuda
* `--num-workers INTEGER RANGE`: Number of workers for pytroch dataloader object.
* `--latent-dim INTEGER RANGE`: Dimension of VAE latent space.
* `--ddo`: Force deletion with confirmation for dataloader object.
* `--dpm`: Force deletion with confirmation for pre-trained VAE model.
* `--cluster-on-raw-data`: Clustering on pc_features dataset, default is set to VAE latent space
* `--cluster-method TEXT`: Clustering method. kmeans or hdbscan; hdbscan is not suitable for latent space of VAE and has some drawbacks for new dataset.
* `--plot-method TEXT`: Plotting method for data. pca or tsne; if you want plot data before clustering on different methods just remove the pc_dataloader.pth with --ddo option.
* `--help`: Show this message and exit.

## `uniXerr develop`

**Usage**:

```console
$ uniXerr develop [OPTIONS]
```

**Options**:

* `--workers INTEGER RANGE`: Number of workers.
* `--help`: Show this message and exit.
