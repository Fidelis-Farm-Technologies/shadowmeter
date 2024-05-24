
# ---------------------------------------------------------------
#
# ---------------------------------------------------------------

FROM nginx:stable-alpine

## Environment
ENV API_HOST=shadowmeter_app
ENV API_PORT=3001
ENV SHADOWMETER_HOST=shadowmeter_app
ENV SHADOWMETER_PORT=3000
ENV SHADOWMETER_SSL=/C=US/ST=Virginia/L=Crozet/O=Fidelis Farm/OU=ShadowMeter/CN=localhost/

## Generate certificate
RUN apk add --update openssl && \
    rm -rf /var/cache/apk/*
RUN openssl req -x509 -out /etc/nginx/ssl.crt -keyout /etc/nginx/ssl.key -newkey rsa:4096 -nodes -sha256 -subj ${SHADOWMETER_SSL}
    
## Copy configuration
COPY ./etc/default.conf /etc/nginx/templates/default.conf.template
COPY ./etc/conf.d /etc/nginx/conf.d

CMD ["nginx", "-g", "daemon off;"]
