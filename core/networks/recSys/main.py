'''
--------------------------------------------------------------------------
  HYBRID CONTENT COLLABORATIVE recSys USING ADVERSARIAL AUTOENCODERS (AAE)
--------------------------------------------------------------------------

        |=> use all personal infos inside users phone for mining process like location, mobile model and etc. <=|
        |=> We don’t want the same input and thus we take a sample from latent vector and put into the decoder to give similar output. <=|


        https://github.com/kilolgupta/Variational-Autoencoders-Collaborative-Filtering
        https://www.kaggle.com/siweiluo/variational-autoencoder-vae-recommendation
        https://www.youtube.com/watch?v=Eeg1DEeWUjA
        https://www.youtube.com/watch?v=u1bpD8Lgp8E
        https://colab.research.google.com/drive/1OOttboitAThA2FKAsUKwJQlMfrdRxVlH#scrollTo=Z-JIkbOWCNt3
        https://developers.google.com/machine-learning/recommendation
        https://developers.google.com/machine-learning/glossary/recsystemshttps
        https://www.youtube.com/watch?v=y_TzOOCJqxI
        https://pdfs.semanticscholar.org/dfad/b5e2e274d3b2de2c203178b5b30cb7a7d0db.pdf
        http://ceur-ws.org/Vol-2367/paper_2.pdf
        https://blog.keras.io/building-autoencoders-in-keras.html
        https://towardsdatascience.com/creating-a-hybrid-content-collaborative-movie-recommender-using-deep-learning-cc8b431618af
        https://github.com/SudharshanShanmugasundaram/Movie-Recommendation-System-using-AutoEncoders
        https://medium.com/snipfeed/how-variational-autoencoders-make-classical-recommender-systems-obsolete-4df8bae51546
        https://towardsdatascience.com/deep-autoencoders-for-collaborative-filtering-6cf8d25bbf1d


        EX : 
                [hey wildonion]
            recommended users near to your position  : @alexa , @rosii, @force
'''

import plotly.graph_objects as go