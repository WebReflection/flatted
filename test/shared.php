<?php

error_reporting(E_ALL | E_STRICT);

require_once(__DIR__ . '/../php/flatted.php');

$str = file_get_contents(__DIR__ . '/shared.json');
$json = Flatted::parse($str);

$arr = $json[0];
$obj = $json[1];

for ($i = 0; $i < 64; $i++) {
  $arr = $arr[0];
  $obj = $obj->obj;
}

assert(count($arr) == 2 && $arr[0] == 'arr' && $arr[1] == 1 && $obj->obj == 2);

assert(Flatted::stringify($json) == $str);

?>
