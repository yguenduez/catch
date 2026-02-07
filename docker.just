

build:
    docker build -t catch .

@run d="-d": build stop
    docker run {{d}} --name catch -p 8111:8111 catch

@stop:
    -docker stop catch
    -docker rm catch
