import os
from collections import namedtuple

import pytest

from tests.support import consts as eve_consts
from tests.support.client import TestClient
from tests.support.server import build_server, kill_server, run_server, build_config
from tests.support.util import next_free_port

PROJECT_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '..'))


ServerInfo = namedtuple('ServerInfo', ('pid', 'port'))


@pytest.fixture(scope='session', autouse=True)
def reefast_server(tmp_path_factory):
    build_server(PROJECT_ROOT)
    config_path = tmp_path_factory.mktemp('reefast_test') / 'config.toml'
    port = next_free_port(8000)
    build_config(path=config_path, port=port)
    pid = run_server(proj_root=PROJECT_ROOT, config_path=config_path)
    try:
        yield ServerInfo(pid, port)
    except Exception:
        kill_server(pid)
        raise
    else:
        kill_server(pid)


@pytest.fixture()
def client(httpserver, reefast_server):
    yield TestClient(httpserver, reefast_server.port)


@pytest.fixture()
def consts():
    yield eve_consts
