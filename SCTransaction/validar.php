<?php
    //conectamos a la base de datos con este php
    include_once('db.php');

    $correo=$_POST['correo'];
    $contraseña=$_POST['contraseña'];

    //usamos la funcion del php que incluimos y lo declaramos como la variable a conectar
    $conectar=conn();

    //validamos datos
    $consulta="SELECT * FROM usuarios_reg WHERE correo='$correo' and contraseña = '$contraseña'";

    //ejecutar consulta
    $resultado=mysqli_query($conectar,$consulta);

    //validacion
    $filas=mysqli_num_rows($resultado);

    if($filas>0){
        header("Location: ../SCTransaction/home.html");
    }
    else{
        echo("Error en la atenticación");
    }

    //liberar espacio de datos tomados
    mysqli_free_result($resultado);
    mysqli_close($conectar);

?>