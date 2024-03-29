target_sources(seal 
    PRIVATE
        ${CMAKE_CURRENT_LIST_DIR}/bigpolyarray.cpp
        ${CMAKE_CURRENT_LIST_DIR}/bigpoly.cpp
        ${CMAKE_CURRENT_LIST_DIR}/biguint.cpp
        ${CMAKE_CURRENT_LIST_DIR}/chooser.cpp
        ${CMAKE_CURRENT_LIST_DIR}/ciphertext.cpp
        ${CMAKE_CURRENT_LIST_DIR}/context.cpp
        ${CMAKE_CURRENT_LIST_DIR}/decryptor.cpp
        ${CMAKE_CURRENT_LIST_DIR}/encoder.cpp
        ${CMAKE_CURRENT_LIST_DIR}/encryptionparams.cpp
        ${CMAKE_CURRENT_LIST_DIR}/encryptor.cpp
        ${CMAKE_CURRENT_LIST_DIR}/evaluationkeys.cpp
        ${CMAKE_CURRENT_LIST_DIR}/evaluator.cpp
        ${CMAKE_CURRENT_LIST_DIR}/galoiskeys.cpp
        ${CMAKE_CURRENT_LIST_DIR}/keygenerator.cpp
        ${CMAKE_CURRENT_LIST_DIR}/plaintext.cpp
        ${CMAKE_CURRENT_LIST_DIR}/polycrt.cpp
        ${CMAKE_CURRENT_LIST_DIR}/randomgen.cpp
        ${CMAKE_CURRENT_LIST_DIR}/simulator.cpp
        ${CMAKE_CURRENT_LIST_DIR}/smallmodulus.cpp
        ${CMAKE_CURRENT_LIST_DIR}/utilities.cpp
)

install(
    FILES
        ${CMAKE_CURRENT_LIST_DIR}/bigpolyarray.h
        ${CMAKE_CURRENT_LIST_DIR}/bigpoly.h
        ${CMAKE_CURRENT_LIST_DIR}/biguint.h
        ${CMAKE_CURRENT_LIST_DIR}/chooser.h
        ${CMAKE_CURRENT_LIST_DIR}/ciphertext.h
        ${CMAKE_CURRENT_LIST_DIR}/context.h
        ${CMAKE_CURRENT_LIST_DIR}/decryptor.h
        ${CMAKE_CURRENT_LIST_DIR}/defaultparams.h
        ${CMAKE_CURRENT_LIST_DIR}/encoder.h
        ${CMAKE_CURRENT_LIST_DIR}/encryptionparams.h
        ${CMAKE_CURRENT_LIST_DIR}/encryptor.h
        ${CMAKE_CURRENT_LIST_DIR}/evaluationkeys.h
        ${CMAKE_CURRENT_LIST_DIR}/evaluator.h
        ${CMAKE_CURRENT_LIST_DIR}/galoiskeys.h
        ${CMAKE_CURRENT_LIST_DIR}/keygenerator.h
        ${CMAKE_CURRENT_LIST_DIR}/memorypoolhandle.h
        ${CMAKE_CURRENT_LIST_DIR}/plaintext.h
        ${CMAKE_CURRENT_LIST_DIR}/polycrt.h
        ${CMAKE_CURRENT_LIST_DIR}/publickey.h
        ${CMAKE_CURRENT_LIST_DIR}/randomgen.h
        ${CMAKE_CURRENT_LIST_DIR}/seal.h
        ${CMAKE_CURRENT_LIST_DIR}/secretkey.h
        ${CMAKE_CURRENT_LIST_DIR}/simulator.h
        ${CMAKE_CURRENT_LIST_DIR}/smallmodulus.h
        ${CMAKE_CURRENT_LIST_DIR}/utilities.h
    DESTINATION
        ${SEAL_INCLUDES_INSTALL_DIR}/seal
)

add_subdirectory(util)