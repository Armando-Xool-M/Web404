<?php
    //Recepcion de datos del formulario
    $nombre=$_POST['nombre'];
    $apellido=$_POST['apellido'];
    $correo=$_POST['correo']; 
    $contraseña=$_POST['contraseña'];

    echo("los datos ingresados son");
    echo("$nombre,$apellido,$correo,$contraseña")
?>