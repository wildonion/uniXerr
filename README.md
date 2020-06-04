<p align="center">
    <img src="https://github.com/wildonion/uniXerr/blob/master/board/drawing/uniXerr_R50.png"
</p>

## Everything can be everything, and everything can turn into everything else.

###### :rice: _if you can't find the pattern is because you don't look closely :grey_exclamation:_

---

# Setup
* Activate _uniXerr_ environment: ```conda activate uniXerr```
* Create the environment from the _uniXerr.yml_ file: ```conda env create -f uniXerr.yml```
* Update the environment using the _uniXerr.yml_ file: ```conda env update -f uniXerr.yml --prune```
* Export your active environment to _uniXerr.yml_ file: ```conda env export > uniXerr.yml```

# Usage
Run `app.py`. This is the main server of the uniXerr protocol and it can be controlled using `eye.py` through `ZMQ` socket. It's a layer on top of the uniXerr core. The whole structure is simaple and understandable and running the code you can understand the whole protocol, 