<?php
    //funcion para conectar
    function conn(){

        $hostname="localhost";
        $usuariodb="root";
        $passworddb="";
        $dbname="usuarios";
    
        //conexion con la base de datos
        $conectar = mysqli_connect($hostname,$usuariodb,$passworddb,$dbname);
        return $conectar;
    }
?>