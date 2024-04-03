import pytest
import oogway


def test_sum_as_string():
    assert oogway.sum_as_string(1, 1) == "2"
