FROM debian

ADD ./target/release/ohdear-application-health /bin/ohdear-application-health

EXPOSE 8000

CMD [ "ohdear-application-health" ]