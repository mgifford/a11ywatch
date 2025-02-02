/// generate central backend services [TODO: ALLOW DEFAULT PORT]
pub fn generate_compose_backend() -> &'static str {
    &r#"
version: '3.9'
services:
  api:
    container_name: api
    image: a11ywatch/a11ywatch-core
    ports:
      - 3280:8080
      - 50051
    depends_on:
      - mongodb
    environment:
      - DB_URL=${DB_URL:-mongodb://mongodb:27017/?compressors=zlib&gssapiServiceName=mongodb}
      - DB_NAME=${DB_NAME:-a11ywatch}
      - APOLLO_GRAPH_VARIANT=${APOLLO_GRAPH_VARIANT:-current}
      - APOLLO_SCHEMA_REPORTING=${APOLLO_SCHEMA_REPORTING:-false}
      - CLIENT_URL=${CLIENT_URL:-http://localhost:3000}
      - GRAPHQL_PORT=${GRAPHQL_PORT:-8080}
      - SCRIPTS_CDN_URL=${SCRIPTS_CDN_URL:-http://localhost:8090/api}
      - ROOT_URL=${ROOT_URL:-http://localhost:3280}
      - REDIS_CLIENT=${REDIS_CLIENT:-redis://redis:6379}
      - REDIS_HOST=redis
      - SUPER_MODE=${SUPER_MODE:-true}
    networks:
      - back-net

  pagemind:
    container_name: pagemind
    image: a11ywatch/pagemind
    ports:
      - 8040
      - 50052
    environment:
      - SCRIPTS_CDN_URL=${SCRIPTS_CDN_URL:-http://127.0.0.1:8090/api}
      - SCRIPTS_CDN_URL_HOST=${SCRIPTS_CDN_URL_HOST:-http://localhost:8090/cdn}
      - PORT=${PAGEMIND_PORT:-8040}
      - PAGESPEED_API_KEY=${PAGESPEED_API_KEY}
    networks:
      - back-net

  mav:
    container_name: mav
    image: a11ywatch/mav
    ports:
      - 8020
      - 50053
    environment:
      - PORT=${MAV_PORT:-8020}
      - COMPUTER_VISION_ENDPOINT=${COMPUTER_VISION_ENDPOINT}
      - COMPUTER_VISION_SUBSCRIPTION_KEY=${COMPUTER_VISION_SUBSCRIPTION_KEY}
    networks:
      - back-net

  cdn-server:
    container_name: cdn-server
    image: a11ywatch/cdn-server
    ports:
      - 8090:8090
      - 50054
    environment:
      - PORT=${ELASTIC_CDN_PORT:-8090}
    networks:
      - back-net

  crawler:
    container_name: crawler
    image: a11ywatch/crawler
    ports:
      - 50055
    networks:
      - back-net

  mongodb:
    container_name: mongodb
    image: mongo
    ports:
      - 27017:27017
    volumes:
      - mongodb:/data/db
    environment:
      - MONGO_INITDB_DATABASE=${DB_NAME:-a11ywatch}
    networks:
      - back-net

  redis:
    container_name: redis
    image: redis:7.0-rc2-alpine
    environment:
      - ALLOW_EMPTY_PASSWORD=yes
    networks:
      - back-net

  chrome:
    image: a11ywatch/chrome
    container_name: chrome
    command:
      [
        chromium-browser,
        '--headless',
        '--no-sandbox',
        '--hide-scrollbars',
        '--mute-audio',
        '--no-zygote',
        '--no-first-run',
        '--remote-debugging-address=0.0.0.0',
        '--remote-debugging-port=9222',
        '--max-wait-for-load=2500',
        '--allow-running-insecure-content',
        '--autoplay-policy=user-gesture-required',
        '--disable-default-apps',
        '--disable-storage-reset',
        '--disable-dev-shm-usage',
        '--disable-http2',
        '--disable-domain-reliability',
        '--disable-component-update',
        '--disable-sync',
        '--disable-background-networking',
        '--disable-background-timer-throttling',
        '--disable-notifications',
        '--disable-accelerated-2d-canvas',
        '--disable-accelerated-video-decode',
        '--disable-extensions',
        '--ignore-certificate-errors',
        '--disable-popup-blocking',
        '--disable-renderer-backgrounding',
        '--disable-client-side-phishing-detection',
        '--disable-setuid-sandbox',
        '--disable-hang-monitor',
        '--disable-features=ScriptStreaming,TranslateUI,BlinkGenPropertyTrees',
        '--disable-backgrounding-occluded-windows',
        '--no-default-browser-check',
        '--metrics-recording-only',
        '--disable-component-extensions-with-background-pages',
      ]
    ports:
      - 9222:9222
    networks:
      - back-net

volumes:
  mongodb:

networks:
  front-net:
  back-net:

  "#
}

/// generate front end client
pub fn generate_compose_frontend() -> &'static str {
    &"
version: '3.9'
services:
  web:
    container_name: web
    image: a11ywatch/web
    ports:
      - 3270:3000
    networks:
      - front-net
    environment:
      - PORT=3000
      - API=${API:-http://localhost:3280/graphql}
      - WEB_SOCKET_URL=${WEB_SOCKET_URL:-ws://localhost:3280/graphql}
    networks:
      - front-net

networks:
  front-net:
  
"
}