from flask import Flask, request, jsonify, Response
import requests

API_KEY: str = "fcaa6a25-c443-4994-bb9b-73416f2a74e5"
API: str = f"key={API_KEY}"

app = Flask(__name__)


@app.route("/weather/<country>/<city>", methods=["GET"])
def get_city_weather(country: str, city: str) -> None:
    ...


@app.route("/supported-countries", methods=["GET"])
def get_all_supported_countries() -> None:
    ENDPOINT: str = f"http://api.airvisual.com/v2/countries?{API}"
    response = requests.get(ENDPOINT)
    return response.json()


# @app.route("/supported-states/<country>", methods=["GET"])
# def get_all_supported_states(country: str) -> None:
#     ENDPOINT: str = f"http://api.airvisual.com/v2/states?country={country}?{API}"
#     response = requests.get(ENDPOINT)
#     return response.json()

@app.route("/supported-cities-in-state/<country>/<state>", methods=["GET"])
def get_all_supported_cities_in_state(country: str, state: str) -> None:
    ENDPOINT: str = f"http://api.airvisual.com/v2/cities?state={state}&country={country}&{API}"
    response = requests.get(ENDPOINT)
    return response.json()


@app.route("/nearest-city", methods=["GET"])
def get_nearest_city() -> None:
    ENDPOINT: str = f"http://api.airvisual.com/v2/nearest_city?{API}"
    response = requests.get(ENDPOINT)
    return response.json()


@app.route("/details/<country>/<state>/<city>", methods=["GET"])
def get_city_details(country: str, state: str, city: str) -> None:
    ENDPOINT: str = f"http://api.airvisual.com/v2/city?city={city}&state={state}&country={country}&{API}"
    response = requests.get(ENDPOINT)
    return response.json()


@app.route("/details/<station>/<country>/<state>/<city>", methods=["GET"])
def get_station_details(station: str, country: str, state: str, city: str) -> None:
    ENDPOINT: str = f"http://api.airvisual.com/v2/station?station={station}&city={city}&state={state}&country={country}&{API}"
    response = requests.get(ENDPOINT)
    return response.json()


@app.route("/ranking", methods=["GET"])
def get_country_ranking() -> None:
    ENDPOINT: str = f"http://api.airvisual.com/v2/city_ranking?{API}"
    response = requests.get(ENDPOINT)
    return response.json()


if __name__ == "__main__":
    app.run(debug=True)
