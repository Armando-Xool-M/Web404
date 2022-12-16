<?php
    include_once('db.php');

    //Recepcion de datos del formulario
    $nombre=$_POST['nombre'];
    $apellido=$_POST['apellido'];
    $correo=$_POST['correo']; 
    $contraseña=$_POST['contraseña'];

    //llenado y conexion con la base de datos
    $conectar= conn(); //conecta con la base de datos
    $sql="INSERT INTO usuarios_reg (correo,nombre,apellido,contraseña)
    VALUES ('$correo','$nombre','$apellido','$contraseña')";
    $result=mysqli_query($conectar,$sql)or trigger_error("Query Faile! SQL - Error: ".mysqli_error($conectar), E_USER_ERROR);
    
    header("Location: ../SCTransaction");
?>