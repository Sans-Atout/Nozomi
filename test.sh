function create_file(){
    echo $TEXT > data/$1
}

function generate_folder(){
    mkdir data/$1
    max=10
    for i in `seq 2 $max`
    do
        echo $TEXT > data/$1/$i.txt
    done
}

function generate(){
    mkdir data/$1
    create_file $1/over_write.txt
    create_file $1/write_error.txt
    chmod -w data/$1/write_error.txt
    create_file $1/erase_method.txt
    generate_folder $1/folder
    echo -e "\033[0;32mOK\033[0;0m Test file generated for $1" 
}

function reverse_chmod(){
    chmod +w data/$1/write_error.txt
}

echo "-----------------------------------------------------------"
echo " Testing nozomi create" 
echo "-----------------------------------------------------------"
export TEXT="Logoden biniou degemer mat an penn ar bed lezenn, koulz speredekañ Konk koll vro ha. Gwelout pegement argoat mestr Ar Releg-Kerhuon dehou torfed Mellag diaes, enni greiz eil goleiñ askorn enno. Bluenn miz blijadur hadañ Landreger digor, digempenn treuziñ voull mont ebet, echu endervezh da drezañ. Kembre toenn birviñ oabl gwenn a pegement barrad medisin, marteze ur bod moulañ pakad c’hwezhañ. Boued dimerc’her n’eus ac’hano butun bouzar, abardaez leun gouelañ kaout marennañ, giz mat kilañ bier. Gwec’h yar Kembre trugarez eviti warnon, outañ warnañ, al ur jod, kroc’hen c’hilpenn Pont-Aven abred. Teurel hepken koar c’hroc’hen eo spontus, diriaou mañ e kreskiñ abred, vatezh tregont Pabu berrloer. Dezhañ wech egistout merenn kazh du, c’henwerzh dale Sant-Nouga va waz, kas poazh arvor pla’hig. Naetaat niver se enor aes krediñ, ya koumoul porpant Groe Landreger, a Malo ken klevout. Eñ digwener hegarat mall bluenn distagañ, ur kasoni mirout meur c’houmanant, magañ evito avat kreskiñ."
echo -e "\033[0;32mOK\033[0;0m exporting TEXT variable" 
create_file rename_file.txt
create_file delete_file.txt
create_file file_length.txt
create_file file_to_erase.txt
echo -e "\033[0;32mOK\033[0;0m Create common file for test" 

generate afssi_5020
generate dod_5022022_me
generate dod_5022022_mece
generate random
generate gutman
generate hmgi_s5
generate rcmp_tssit_ops_ii

echo -e "\033[0;32mOK\033[0;0m create file for test"
generate_folder folder_to_erase
echo -e "\033[0;32mOK\033[0;0m create folder for test" 
export TEXT
echo -e "\033[0;32mOK\033[0;0m unset TEST variable" 
echo "-----------------------------------------------------------"
cargo nextest run

echo "-----------------------------------------------------------"
echo " Post Test cleanup" 
echo "-----------------------------------------------------------"
reverse_chmod afssi_5020
reverse_chmod dod_5022022_me
reverse_chmod dod_5022022_mece
reverse_chmod random
reverse_chmod gutman
reverse_chmod hmgi_s5
reverse_chmod rcmp_tssit_ops_ii
echo -e "\033[0;32mOK\033[0;0m reverse chmod -w command" 

rm -r data/*
echo -e "\033[0;32mOK\033[0;0m remove all test data file" 
