FROM library/debian:testing
LABEL author="Michael Fletcher <m.fletcher@theplanet.ca>"
USER root
RUN apt-get update && apt-get install -y unzip libpq5
ADD target/release/birthdays-backend /opt/birthdays-backend
RUN chmod a+rwx /opt/birthdays-backend
EXPOSE 8111
ENTRYPOINT /opt/birthdays-backend
