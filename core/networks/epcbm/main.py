'''
----------------------------------------------------------------------------------------------
  ESTIMATE & PREDICT THE POSITIONS , COINS , BEHAVIOUR ACTIVITY , SEMESTER STATUS & MARKS 
    FOR uniXerr USERS BASED ON THEIR PAST EVENTS AND CLUSTER USING RNNs (LSTM/GRU) & GAN (AAE)
----------------------------------------------------------------------------------------------
              |text <-> speech & natural language processing with BERT & GPT-2|
        
        
        https://stackabuse.com/time-series-prediction-using-lstm-with-pytorch-in-python/
        https://www.curiousily.com/posts/time-series-forecasting-with-lstm-for-daily-coronavirus-cases/
        https://www.kaggle.com/amirrezaeian/time-series-data-analysis-using-lstm-tutorial
        https://stackabuse.com/time-series-analysis-with-lstm-using-pythons-keras-library/
        https://stackabuse.com/time-series-analysis-with-lstm-using-pythons-keras-library/
        https://towardsdatascience.com/time-series-analysis-visualization-forecasting-with-lstm-77a905180eba
        https://romanorac.github.io/machine/learning/2019/09/27/time-series-prediction-with-lstm.html
        https://www.tensorflow.org/tutorials/structured_data/time_series
        https://machinelearningmastery.com/how-to-develop-lstm-models-for-time-series-forecasting/
        https://machinelearningmastery.com/time-series-prediction-lstm-recurrent-neural-networks-python-keras/
        https://missinglink.ai/guides/tensorflow/tensorflow-speech-recognition-two-quick-tutorials/
        https://github.com/mozilla/TTS
        https://github.com/r9y9/nnmnkwii
        https://towardsdatascience.com/you-can-now-speak-using-someone-elses-voice-with-deep-learning-8be24368fa2b
        https://towardsdatascience.com/speech-recognition-analysis-f03ff9ce78e9
        https://medium.com/@fortes.arthur/hands-on-speech-recognition-engine-with-keras-and-python-c60488ac53cd
        https://github.com/israelg99/deepvoice
        https://pythonawesome.com/a-keras-implementation-of-speach-to-text-architectures/
        https://medium.com/manash-en-blog/building-a-dead-simple-word-recognition-engine-using-convnet-in-keras-25e72c19c12b
        https://machinelearningmastery.com/sequence-classification-lstm-recurrent-neural-networks-python-keras/
        https://www.kaggle.com/carlolepelaars/bidirectional-lstm-for-audio-labeling-with-keras
        https://becominghuman.ai/how-to-train-a-neural-network-to-code-by-itself-a432e8a120df
        https://towardsdatascience.com/how-to-build-a-recurrent-neural-network-to-detect-fake-news-35953c19cf0b
        https://www.analyticsvidhya.com/blog/2019/12/detect-fight-neural-fake-news-nlp/
        https://pathmind.com/wiki/word2vec
        https://www.youtube.com/watch?v=64qSgA66P-8
        https://adventuresinmachinelearning.com/word2vec-keras-tutorial/
        https://towardsdatascience.com/deep-learning-for-natural-language-processing-using-word2vec-keras-d9a240c7bb9d
        https://spacy.io/universe/project/spacy-stanza/
        https://machinetalk.org/2019/03/29/neural-machine-translation-with-attention-mechanism/
        https://blog.exxactcorp.com/getting-started-with-nlp-using-the-tensorflow-and-keras-frameworks/
        https://machinelearningmastery.com/tensorflow-tutorial-deep-learning-with-tf-keras/
        https://www.tensorflow.org/tutorials/text/text_classification_rnn
        https://colab.research.google.com/drive/1nQYJq1f7f4R0yeZOzQ9rBKgk00AfLoS0#scrollTo=Uw4JkktB9ceM
        https://towardsdatascience.com/natural-language-processing-from-basics-to-using-rnn-and-lstm-ef6779e4ae66
        https://stackabuse.com/solving-sequence-problems-with-lstm-in-keras/
        https://mc.ai/lstm-with-keras/
        https://adventuresinmachinelearning.com/keras-lstm-tutorial/
        https://bert-embedding.readthedocs.io/en/latest/
        http://jalammar.github.io/illustrated-bert/
        https://mccormickml.com/2019/05/14/BERT-word-embeddings-tutorial/
        https://towardsdatascience.com/comparing-transformer-tokenizers-686307856955
        https://towardsdatascience.com/from-pre-trained-word-embeddings-to-pre-trained-language-models-focus-on-bert-343815627598
        http://jalammar.github.io/a-visual-guide-to-using-bert-for-the-first-time/
        https://towardsdatascience.com/how-to-do-text-binary-classification-with-bert-f1348a25d905
        https://towardsdatascience.com/bert-text-classification-in-3-lines-of-code-using-keras-264db7e7a358
        https://peltarion.com/knowledge-center/tutorials/bert-movie-review-sentiment-analysis
        https://medium.com/swlh/a-simple-guide-on-using-bert-for-text-classification-bbf041ac8d04
        https://medium.com/huggingface/multi-label-text-classification-using-bert-the-mighty-transformer-69714fa3fb3d
        https://www.analyticsvidhya.com/blog/2019/09/demystifying-bert-groundbreaking-nlp-framework/
        https://towardsdatascience.com/nlp-extract-contextualized-word-embeddings-from-bert-keras-tf-67ef29f60a7b
        https://towardsdatascience.com/word-embedding-using-bert-in-python-dd5a86c00342
        https://mc.ai/natural-language-generation-using-bert/
        https://medium.com/saarthi-ai/bert-how-to-build-state-of-the-art-language-models-59dddfa9ac5d
        https://towardsdatascience.com/bert-in-keras-with-tensorflow-hub-76bcbc9417b
        https://medium.com/analytics-vidhya/bert-in-keras-tensorflow-2-0-using-tfhub-huggingface-81c08c5f81d8
        https://androidkt.com/simple-text-classification-using-bert-in-tensorflow-keras-2-0/
        https://towardsdatascience.com/fine-tuning-bert-with-keras-and-tf-module-ed24ea91cff2
        https://www.kaggle.com/igetii/bert-keras
        https://www.kdnuggets.com/2020/02/intent-recognition-bert-keras-tensorflow.html
        https://colab.research.google.com/github/tensorflow/tpu/blob/master/tools/colab/bert_finetuning_with_cloud_tpus.ipynb
        https://medium.com/@brn.pistone/bert-fine-tuning-for-tensorflow-2-0-with-keras-api-9913fc1348f6
        https://stackabuse.com/text-classification-with-bert-tokenizer-and-tf-2-0-in-python/
        https://mc.ai/nlp-extract-contextualized-word-embeddings-from-bert-keras-tf/
        https://medium.com/henry-jia/how-to-score-your-credit-1c08dd73e2ed
        https://www.youtube.com/watch?v=_iag_If4yYA
        https://www.youtube.com/watch?v=ULWLleBDCEY
        https://www.youtube.com/watch?v=xvqsFTUsOmc
        https://towardsdatascience.com/language-translation-with-rnns-d84d43b40571
        https://xiandong79.github.io/seq2seq-%E5%9F%BA%E7%A1%80%E7%9F%A5%E8%AF%86
        https://galaxyproject.github.io/training-material/topics/statistics/tutorials/age-prediction-with-ml/tutorial.html
        https://medium.com/@chataks93/predicting-human-behaviour-activity-using-deep-learning-lstm-fff9030b82e7
        https://arxiv.org/pdf/1708.08744.pdf
        https://arxiv.org/pdf/1804.07405.pdf
        https://www.ritchieng.com/machine-learning-project-student-intervention/
        https://towardsdatascience.com/predicting-students-grades-on-kaggle-fd6ac9b1bfb9


        EX :
                    [hey wildonion]
                weekly positon                           : [A.....B..*..C.....D]
                total mined coins                        : 45
                estimated position                       : A scope
                estimated coin to mine                   : 10
                estimated marks for next quizzes         : [NLA : 80 , CSA : 45 , MATH-1 : 78]
                estimated mark for semester courses exam : [NLA : 43 , CSA : 23 , MATH-1 : 34]
                monthly estimated position               : C scope
                total semester estimated position        : B scope
                interests                                : parkour , music-rock , paiting 
                characteristic                           : generosity , integrity and loving
                age                                      : 22
                semester status                          : passed

'''

import plotly.graph_objects as go
