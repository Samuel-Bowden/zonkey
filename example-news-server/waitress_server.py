import server
from waitress import serve
serve(server.app, host='0.0.0.0', port=8000)