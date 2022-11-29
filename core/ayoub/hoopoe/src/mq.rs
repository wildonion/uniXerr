




/*





    mq clients in rust and js
        | 
        -------- hoopoe mq producer and consumer actor streamer -------- conse hyper server 
                        |                             |
                        |                             -------- mongodb
                        |
                        <---tcp socket--> |broker actor streamer on VPS <---routing channel exchange--> job or task queue buffer| 
                                                                                                            |
                                                                                                            |
                                                                                                            |
                                                                                                            <---mpsc channel---> worker threadpools




    https://www.cloudamqp.com/blog/part1-rabbitmq-for-beginners-what-is-rabbitmq.html#exchanges


    • Producer: Application that sends the messages.
    • Consumer: Application that receives the messages.
    • Queue: Buffer that stores messages.
    • Message: Information that is sent from the producer to a consumer through RabbitMQ.
    • Connection: A TCP connection between your application and the RabbitMQ broker.
    • Channel: A virtual connection inside a connection. When publishing or consuming messages from a queue - it's all done over a channel.
    • Exchange: Receives messages from producers and pushes them to queues depending on rules defined by the exchange type. To receive messages, a queue needs to be bound to at least one exchange.
    • Binding: A binding is a link between a queue and an exchange.
    • Routing key: A key that the exchange looks at to decide how to route the message to queues. Think of the routing key like an address for the message.
    • AMQP: Advanced Message Queuing Protocol is the protocol used by RabbitMQ for messaging.



    mq is actually a tcp socket channel based on actor desing pattern that will send and receive buffers like any other socket channels
    but the only difference between others is it can manage incoming payloads in a specific manner like:
        • it uses an async job or task queue like mpsc jobq channel and celery algos to communicating between actors' threads (send and receive tasks and messages between their worker threadpools)  
        • it has a batch handler which means it can take a batch of tasks and publish them to the producers from the queue
        • receiving only a specific message on a specific topic (receivers can only subscribe to a specific topic)
        • schduling a message to be sent later using a task queue handler
        • schduling a message to be received at a specific condition using a task queue handler
        • sending and broadcasting message only to specific receivers 
        • handle (send and receive) tasks and messages asyncly inside a threadpool
        • buffering messages inside a queue to send them once the receiver gets backed online





*/



pub mod app;