echo "-----------------------------------------------------------"
echo " Testing nozomi create" 
echo "-----------------------------------------------------------"
export TEXT="Logoden biniou degemer mat an penn ar bed lezenn, koulz speredekañ Konk koll vro ha. Gwelout pegement argoat mestr Ar Releg-Kerhuon dehou torfed Mellag diaes, enni greiz eil goleiñ askorn enno. Bluenn miz blijadur hadañ Landreger digor, digempenn treuziñ voull mont ebet, echu endervezh da drezañ. Kembre toenn birviñ oabl gwenn a pegement barrad medisin, marteze ur bod moulañ pakad c’hwezhañ. Boued dimerc’her n’eus ac’hano butun bouzar, abardaez leun gouelañ kaout marennañ, giz mat kilañ bier. Gwec’h yar Kembre trugarez eviti warnon, outañ warnañ, al ur jod, kroc’hen c’hilpenn Pont-Aven abred. Teurel hepken koar c’hroc’hen eo spontus, diriaou mañ e kreskiñ abred, vatezh tregont Pabu berrloer. Dezhañ wech egistout merenn kazh du, c’henwerzh dale Sant-Nouga va waz, kas poazh arvor pla’hig. Naetaat niver se enor aes krediñ, ya koumoul porpant Groe Landreger, a Malo ken klevout. Eñ digwener hegarat mall bluenn distagañ, ur kasoni mirout meur c’houmanant, magañ evito avat kreskiñ."
echo -e "\033[0;32mok\033[0;0m exporting TEXT variable" 
rm -r data/*
echo -e "\033[0;32mok\033[0;0m remove all test data file" 
echo $TEXT > ./data/rename_file.txt
echo $TEXT > ./data/delete_file.txt
echo $TEXT > ./data/zero_overwrite.txt
echo $TEXT > ./data/gutmann_overwrite_file.txt
echo $TEXT > ./data/hmgi_s5_overwrite_file.txt
echo $TEXT > ./data/rcmp_tssit_ops_ii_overwrite_file.txt
echo $TEXT > ./data/afssi_5020_overwrite_file.txt
echo $TEXT > ./data/dod_522022_mece_overwrite_file.txt
echo $TEXT > ./data/dod_522022_me_overwrite_file.txt
echo $TEXT > ./data/file_overwriting_hexa.txt
echo $TEXT > ./data/file_length.txt
echo $TEXT > ./data/file_1.txt
echo -e "\033[0;32mok\033[0;0m create file for test"
mkdir ./data/folder
echo $TEXT > ./data/folder/file_1.txt
echo $TEXT > ./data/folder/file_2.txt
echo $TEXT > ./data/folder/file_3.txt
echo -e "\033[0;32mok\033[0;0m create folder for test" 
export TEXT
echo -e "\033[0;32mok\033[0;0m unset TEST variable" 
echo "-----------------------------------------------------------"
cargo test