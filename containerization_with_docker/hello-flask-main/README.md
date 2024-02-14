# hello-flask

An example **Hello World!** application created with *Python* and *Flask* I use for practicing
containerizing applications and cloud deployments.

## How To Use This Project

This repo contains a Flask application self-contained in the `hello.py` file and exposes a single endpoint `/`.

To run the application you need to:

1. Create a Python virtual environment (recommended but not mandatory):

    `python -m venv ~/.hello-flask`

    and activate it with:

    `source ~/.hello-flask/bin/activate`

2. Install Python pip requirements:

    `pip install -r requirements.txt`

3. Run the Flask application:

    with `python hello.py`

    or use Flask environment `flask run`
